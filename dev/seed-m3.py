#!/usr/bin/env python3
"""Seeds M3 cases in the room Design (dev/seed.sh and seed-m2.py first): a poll with votes, a
pinned message, and bob typing (pass --typing to only send the typing notice, for 30 s).
Local test values only."""
import json
import sys
import time
import urllib.parse
import urllib.request

HS = "http://localhost:18008"
TXN = int(time.time() * 1000)


def call(method, path, token=None, body=None):
    request = urllib.request.Request(HS + path, data=json.dumps(body).encode() if body is not None else None, method=method)
    request.add_header("Content-Type", "application/json")
    if token:
        request.add_header("Authorization", f"Bearer {token}")
    with urllib.request.urlopen(request) as response:
        return json.loads(response.read() or b"{}")


def login(user):
    return call("POST", "/_matrix/client/v3/login", body={
        "type": "m.login.password", "identifier": {"type": "m.id.user", "user": user},
        "password": f"{user}-dev-password"})["access_token"]


def room_named(token, name):
    for room_id in call("GET", "/_matrix/client/v3/joined_rooms", token)["joined_rooms"]:
        try:
            state = call("GET", f"/_matrix/client/v3/rooms/{urllib.parse.quote(room_id)}/state/m.room.name/", token)
        except urllib.error.HTTPError:
            continue
        if state.get("name") == name:
            return room_id
    raise SystemExit(f"no room named {name}")


def send(token, room, kind, content):
    global TXN
    TXN += 1
    return call("PUT", f"/_matrix/client/v3/rooms/{urllib.parse.quote(room)}/send/{kind}/m3{TXN}", token, content)["event_id"]


alice, bob, carol = login("alice"), login("bob"), login("carol")
design = room_named(bob, "Design")
q = urllib.parse.quote(design)

if "--typing" in sys.argv:
    call("PUT", f"/_matrix/client/v3/rooms/{q}/typing/@bob:localhost", bob, {"typing": True, "timeout": 30000})
    print("bob is typing in Design for 30 s")
    raise SystemExit

answers = [{"id": "a1", "org.matrix.msc1767.text": "Outlined icons"}, {"id": "a2", "org.matrix.msc1767.text": "Filled icons"},
           {"id": "a3", "org.matrix.msc1767.text": "Keep the web's set"}]
poll = send(bob, design, "org.matrix.msc3381.poll.start", {
    "org.matrix.msc3381.poll.start": {
        "question": {"org.matrix.msc1767.text": "Which icon style for the desktop app?"},
        "kind": "org.matrix.msc3381.poll.disclosed", "max_selections": 1, "answers": answers},
    "org.matrix.msc1767.text": "Which icon style for the desktop app?",
})
for token, answer in ((bob, "a1"), (alice, "a3")):
    send(token, design, "org.matrix.msc3381.poll.response", {
        "m.relates_to": {"rel_type": "m.reference", "event_id": poll},
        "org.matrix.msc3381.poll.response": {"answers": [answer]},
    })

important = send(bob, design, "m.room.message", {"msgtype": "m.text", "body": "Release checklist: build, sign, notarise, publish."})
call("PUT", f"/_matrix/client/v3/rooms/{q}/state/m.room.pinned_events/", alice, {"pinned": [important]})
print("Seeded a poll with votes and a pinned message in Design")
