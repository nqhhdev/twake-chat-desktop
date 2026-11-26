#!/usr/bin/env bash
# Start the local Synapse and create the test users (local test values only).
set -euo pipefail
cd "$(dirname "$0")"
if [ ! -f synapse/.signing-generated ]; then
  docker compose run --rm --entrypoint sh synapse -c \
    'test -f /data/localhost.signing.key || python -m synapse.app.homeserver --config-path=/config/homeserver.yaml --generate-keys; chown -R 991:991 /data' >/dev/null
  touch synapse/.signing-generated
fi
# The certificate of the openid listener (calls: the token service checks over HTTPS)
if [ ! -f synapse/tls/localhost.crt ]; then
  mkdir -p synapse/tls
  openssl req -x509 -newkey rsa:2048 -nodes -days 3650 -subj /CN=localhost \
    -keyout synapse/tls/localhost.key -out synapse/tls/localhost.crt 2>/dev/null
  chmod 644 synapse/tls/localhost.key
fi
docker compose up -d --wait
for u in alice bob carol perf; do
  docker compose exec -T synapse register_new_matrix_user -c /config/homeserver.yaml \
    -u "$u" -p "${u}-dev-password" --no-admin http://localhost:8008 2>/dev/null || true
done
echo "Synapse up on http://localhost:18008 (users: alice, bob, carol; password: <user>-dev-password)"
