#!/usr/bin/env bash
# Seeds the local Synapse (dev/up.sh) with rooms and messages for alice, bob and carol.
# Local test values only. Run once after up.sh; running again adds more rooms.
set -euo pipefail
HS=${HS:-http://localhost:18008}

login() {
  curl -fsS -X POST "$HS/_matrix/client/v3/login" -H 'Content-Type: application/json' \
    -d "{\"type\":\"m.login.password\",\"identifier\":{\"type\":\"m.id.user\",\"user\":\"$1\"},\"password\":\"$1-dev-password\",\"initial_device_display_name\":\"seed\"}" |
    python3 -c 'import json,sys; print(json.load(sys.stdin)["access_token"])'
}

# create_room TOKEN JSON -> room id
create_room() {
  curl -fsS -X POST "$HS/_matrix/client/v3/createRoom" -H "Authorization: Bearer $1" \
    -H 'Content-Type: application/json' -d "$2" |
    python3 -c 'import json,sys; print(json.load(sys.stdin)["room_id"])'
}

join() {
  curl -fsS -X POST "$HS/_matrix/client/v3/join/$2" -H "Authorization: Bearer $1" \
    -H 'Content-Type: application/json' -d '{}' >/dev/null
}

TXN=$(date +%s)
# send TOKEN ROOM BODY [HTML]
send() {
  TXN=$((TXN + 1))
  local payload
  payload=$(python3 -c 'import json,sys
body, html = sys.argv[1], sys.argv[2]
c = {"msgtype": "m.text", "body": body}
if html: c.update(format="org.matrix.custom.html", formatted_body=html)
print(json.dumps(c))' "$3" "${4:-}")
  curl -fsS -X PUT "$HS/_matrix/client/v3/rooms/$2/send/m.room.message/seed$TXN" \
    -H "Authorization: Bearer $1" -H 'Content-Type: application/json' -d "$payload" >/dev/null
}

ALICE=$(login alice)
BOB=$(login bob)
CAROL=$(login carol)

GENERAL=$(create_room "$ALICE" '{"name":"General","preset":"private_chat","invite":["@bob:localhost","@carol:localhost"]}')
join "$BOB" "$GENERAL"
join "$CAROL" "$GENERAL"
DESIGN=$(create_room "$ALICE" '{"name":"Design","topic":"Mockups and reviews","preset":"private_chat","invite":["@bob:localhost"]}')
join "$BOB" "$DESIGN"
DM=$(create_room "$BOB" '{"preset":"trusted_private_chat","is_direct":true,"invite":["@alice:localhost"]}')
join "$ALICE" "$DM"

send "$ALICE" "$GENERAL" "Welcome to the desktop app test room"
send "$BOB" "$GENERAL" "Formatting check: bold, italic, strike, code, link" \
  '<b>Bold</b>, <i>italic</i>, <del>strike</del>, <code>cargo run</code> and <a href="https://twake.app">a link</a>'
send "$CAROL" "$GENERAL" "A list" '<p>Release steps:</p><ol><li>build</li><li>sign</li><li>ship</li></ol><ul><li>macOS</li><li>Windows</li><li>Linux</li></ul>'
send "$ALICE" "$GENERAL" "> quoted\nanswer" '<blockquote>Can we ship on Friday?</blockquote><p>Yes, if CI is green.</p>'
send "$BOB" "$GENERAL" "code block" '<pre><code>fn main() {\n    println!("hello");\n}\n</code></pre>'
send "$CAROL" "$GENERAL" "# not a heading, 1. not a list, *not bold*, see https://matrix.org/docs."
for i in $(seq 1 60); do
  who=$([ $((i % 3)) -eq 0 ] && echo "$ALICE" || ([ $((i % 3)) -eq 1 ] && echo "$BOB" || echo "$CAROL"))
  send "$who" "$GENERAL" "History message $i: scrolling up should load older pages."
done
send "$BOB" "$GENERAL" "Last message of the seed 🎉"

send "$ALICE" "$DESIGN" "New mockups are in the Drive folder"
send "$BOB" "$DESIGN" "Looks great! A few notes on the composer." '<p>Looks great! A few notes on the <b>composer</b>:</p><ul><li>bigger send button</li><li>keep the formats bar folded</li></ul>'

send "$BOB" "$DM" "Hi Alice, do you have a minute?"
send "$ALICE" "$DM" "Sure, what's up?"

echo "Seeded: General ($GENERAL), Design ($DESIGN), DM ($DM)"
