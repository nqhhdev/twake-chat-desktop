#!/usr/bin/env python3
"""Seeds M4 Activity cases in the room Design while the app runs (dev/seed.sh, seed-m2.py
and seed-m3.py first): bob mentions alice (m.mentions), reacts to alice's last message,
and replies in a thread alice started. Usage: dev/seed-m4.py [delay-seconds].
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
    return call("PUT", f"/_matrix/client/v3/rooms/{urllib.parse.quote(room)}/send/{kind}/m4{TXN}", token, content)["event_id"]


time.sleep(float(sys.argv[1]) if len(sys.argv) > 1 else 0)
alice, bob = login("alice"), login("bob")
design = room_named(bob, "Design")
q = urllib.parse.quote(design)

send(bob, design, "m.room.message", {
    "msgtype": "m.text", "body": "alice: can you review the activity screen?",
    "format": "org.matrix.custom.html",
    "formatted_body": '<a href="https://matrix.to/#/@alice:localhost">alice</a>: can you review the activity screen?',
    "m.mentions": {"user_ids": ["@alice:localhost"]}})

chunk = call("GET", f"/_matrix/client/v3/rooms/{q}/messages?dir=b&limit=50", bob)["chunk"]
own = [e for e in chunk if e["type"] == "m.room.message" and e["sender"] == "@alice:localhost"
       and "m.relates_to" not in e["content"]]
if own:
    send(bob, design, "m.reaction", {"m.relates_to": {"rel_type": "m.annotation", "event_id": own[0]["event_id"], "key": "🎉"}})

root = send(alice, design, "m.room.message", {"msgtype": "m.text", "body": "Thread: activity screen layout"})
send(alice, design, "m.room.message", {"msgtype": "m.text", "body": "Rail button or a menu item?",
                                       "m.relates_to": {"rel_type": "m.thread", "event_id": root}})
time.sleep(1)
send(bob, design, "m.room.message", {"msgtype": "m.text", "body": "Both: rail button and Cmd+Shift+A",
                                     "m.relates_to": {"rel_type": "m.thread", "event_id": root}})
print("Seeded a mention, a reaction and a thread reply in Design")
