#!/usr/bin/env bash
# The large account of the perf measurements (ADR-010), like the web's
# e2e/scripts/seed-large.mjs: user `perf` in 300 rooms, one message each.
# Local test values only. Needs dev/up.sh (which registers `perf`).
set -euo pipefail
HS=${HS:-http://localhost:18008}
COUNT=${COUNT:-300}
TOKEN=$(curl -fsS -X POST "$HS/_matrix/client/v3/login" -H 'Content-Type: application/json' \
  -d '{"type":"m.login.password","identifier":{"type":"m.id.user","user":"perf"},"password":"perf-dev-password","initial_device_display_name":"seed"}' |
  python3 -c 'import json,sys; print(json.load(sys.stdin)["access_token"])')
one() {
  local i=$1 room
  room=$(curl -fsS -X POST "$HS/_matrix/client/v3/createRoom" -H "Authorization: Bearer $TOKEN" \
    -H 'Content-Type: application/json' -d "{\"name\":\"Channel $i\",\"preset\":\"private_chat\"}" |
    python3 -c 'import json,sys; print(json.load(sys.stdin)["room_id"])')
  curl -fsS -X PUT "$HS/_matrix/client/v3/rooms/$room/send/m.room.message/seed$i" -H "Authorization: Bearer $TOKEN" \
    -H 'Content-Type: application/json' -d "{\"msgtype\":\"m.text\",\"body\":\"First message of channel $i\"}" >/dev/null
}
for i in $(seq "${FROM:-1}" "$COUNT"); do
  one "$i" &
  if (( i % 8 == 0 )); then wait; fi
done
wait
echo "perf is in $COUNT rooms"
