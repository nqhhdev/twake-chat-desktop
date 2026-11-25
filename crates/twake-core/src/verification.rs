//! Verifying one's own devices by comparing emojis (web `DeviceVerificationDialog`, the
//! `verification` use cases): asked from this device, or by another one; one at a time.

use std::sync::{Arc, Mutex};

use futures_util::StreamExt;
use matrix_sdk::Client;
use matrix_sdk::encryption::verification::{
    SasState, SasVerification, Verification, VerificationRequest, VerificationRequestState,
};
use matrix_sdk::ruma::events::key::verification::request::ToDeviceKeyVerificationRequestEvent;

use crate::engine::{Emit, Event};

/// Where a verification stands (web `VerificationPhase`, without the QR code ones).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerificationPhase {
    /// Another device asks to be verified.
    Incoming,
    /// This device asked; no other device answered yet.
    Asking,
    /// Accepted: the emojis are on their way.
    Waiting,
    /// The emojis (symbol, English name) to compare, or the numbers when there are none.
    Comparing {
        emojis: Vec<(String, String)>,
        decimals: (u16, u16, u16),
    },
    Done,
    Cancelled,
}

/// The verification under way, if any.
#[derive(Clone, Default)]
pub struct Verifier {
    request: Arc<Mutex<Option<VerificationRequest>>>,
    sas: Arc<Mutex<Option<SasVerification>>>,
}

impl Verifier {
    fn set(&self, request: Option<VerificationRequest>) {
        if let Ok(mut r) = self.request.lock() {
            *r = request;
        }
        if let Ok(mut s) = self.sas.lock() {
            *s = None;
        }
    }

    fn request(&self) -> Option<VerificationRequest> {
        self.request.lock().ok().and_then(|r| r.clone())
    }

    fn sas(&self) -> Option<SasVerification> {
        self.sas.lock().ok().and_then(|s| s.clone())
    }

    /// Hears the requests of the user's other devices.
    pub fn listen(&self, client: &Client, emit: Emit) {
        let verifier = self.clone();
        client.add_event_handler(
            move |event: ToDeviceKeyVerificationRequestEvent, client: Client| {
                let verifier = verifier.clone();
                let emit = emit.clone();
                async move {
                    // Only one's own devices, and one verification at a time
                    if client.user_id() != Some(event.sender.as_ref())
                        || verifier.request().is_some()
                    {
                        return;
                    }
                    let Some(request) = client
                        .encryption()
                        .get_verification_request(&event.sender, &event.content.transaction_id)
                        .await
                    else {
                        return;
                    };
                    verifier.set(Some(request.clone()));
                    emit(Event::Verification(VerificationPhase::Incoming));
                    tokio::spawn(verifier.follow(request, emit));
                }
            },
        );
    }

    /// « Verify with another device »: asks the user's other devices.
    pub async fn start(&self, client: &Client, emit: Emit) -> Result<(), String> {
        let own = client.user_id().ok_or("not signed in")?.to_owned();
        let identity = client
            .encryption()
            .get_user_identity(&own)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("no identity for this account yet")?;
        let request = identity
            .request_verification()
            .await
            .map_err(|e| e.to_string())?;
        self.set(Some(request.clone()));
        emit(Event::Verification(VerificationPhase::Asking));
        tokio::spawn(self.clone().follow(request, emit));
        Ok(())
    }

    /// « Verify » on a request of another device.
    pub async fn accept(&self) -> Result<(), String> {
        let request = self.request().ok_or("no verification")?;
        request.accept().await.map_err(|e| e.to_string())
    }

    /// « Not now », a close, or « They differ » before the emojis: the verification stops.
    pub async fn cancel(&self) {
        if let Some(sas) = self.sas() {
            let _ = sas.cancel().await;
        } else if let Some(request) = self.request() {
            let _ = request.cancel().await;
        }
        self.set(None);
    }

    /// « They match » or « They differ ».
    pub async fn confirm(&self, matching: bool) -> Result<(), String> {
        let sas = self.sas().ok_or("no emojis to compare")?;
        if matching {
            sas.confirm().await.map_err(|e| e.to_string())
        } else {
            sas.mismatch().await.map_err(|e| e.to_string())
        }
    }

    /// Follows a request to its end; the device that asked starts the emojis once the other
    /// is ready, the other accepts them.
    async fn follow(self, request: VerificationRequest, emit: Emit) {
        let mut changes = request.changes();
        while let Some(state) = changes.next().await {
            match state {
                VerificationRequestState::Ready { .. } => {
                    emit(Event::Verification(VerificationPhase::Waiting));
                    if request.we_started()
                        && let Ok(Some(sas)) = request.start_sas().await
                    {
                        self.follow_sas(sas, false, emit.clone()).await;
                        return;
                    }
                }
                VerificationRequestState::Transitioned {
                    verification: Verification::SasV1(sas),
                } => {
                    self.follow_sas(sas, true, emit.clone()).await;
                    return;
                }
                VerificationRequestState::Done => {
                    emit(Event::Verification(VerificationPhase::Done));
                    self.set(None);
                    return;
                }
                VerificationRequestState::Cancelled(_) => {
                    emit(Event::Verification(VerificationPhase::Cancelled));
                    self.set(None);
                    return;
                }
                _ => {}
            }
        }
    }

    async fn follow_sas(&self, sas: SasVerification, accept: bool, emit: Emit) {
        if let Ok(mut s) = self.sas.lock() {
            *s = Some(sas.clone());
        }
        if accept {
            let _ = sas.accept().await;
        }
        let mut changes = sas.changes();
        while let Some(state) = changes.next().await {
            match state {
                SasState::KeysExchanged { emojis, decimals } => {
                    let emojis = emojis
                        .map(|e| {
                            e.emojis
                                .iter()
                                .map(|emoji| {
                                    (emoji.symbol.to_string(), emoji.description.to_string())
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    emit(Event::Verification(VerificationPhase::Comparing {
                        emojis,
                        decimals,
                    }));
                }
                SasState::Done { .. } => {
                    emit(Event::Verification(VerificationPhase::Done));
                    self.set(None);
                    return;
                }
                SasState::Cancelled(_) => {
                    emit(Event::Verification(VerificationPhase::Cancelled));
                    self.set(None);
                    return;
                }
                _ => {}
            }
        }
    }
}
