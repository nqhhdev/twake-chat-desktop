#!/usr/bin/env python3
"""Seeds M2 cases in the room Design (dev/seed.sh first): reactions, a reply, an edit, a
deleted message and a thread with replies. Local test values only."""
import json
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


def send(token, room, kind, content):
    global TXN
    TXN += 1
    room_q = urllib.parse.quote(room)
    return call("PUT", f"/_matrix/client/v3/rooms/{room_q}/send/{kind}/m2{TXN}", token, content)["event_id"]


def room_named(token, name):
    for room_id in call("GET", "/_matrix/client/v3/joined_rooms", token)["joined_rooms"]:
        try:
            state = call("GET", f"/_matrix/client/v3/rooms/{urllib.parse.quote(room_id)}/state/m.room.name/", token)
        except urllib.error.HTTPError:
            continue
        if state.get("name") == name:
            return room_id
    raise SystemExit(f"no room named {name}")


alice, bob = login("alice"), login("bob")
design = room_named(bob, "Design")

question = send(bob, design, "m.room.message", {"msgtype": "m.text", "body": "Which icon set for the composer?"})
for token, key in ((alice, "👍"), (bob, "👍"), (alice, "💜")):
    send(token, design, "m.reaction", {"m.relates_to": {"rel_type": "m.annotation", "event_id": question, "key": key}})

send(alice, design, "m.room.message", {
    "msgtype": "m.text", "body": "The outlined one, like the web app.",
    "m.relates_to": {"m.in_reply_to": {"event_id": question}},
})

typo = send(bob, design, "m.room.message", {"msgtype": "m.text", "body": "Shiping on Fridy"})
send(bob, design, "m.room.message", {
    "msgtype": "m.text", "body": "* Shipping on Friday",
    "m.new_content": {"msgtype": "m.text", "body": "Shipping on Friday"},
    "m.relates_to": {"rel_type": "m.replace", "event_id": typo},
})

oops = send(alice, design, "m.room.message", {"msgtype": "m.text", "body": "Wrong room, sorry"})
call("PUT", f"/_matrix/client/v3/rooms/{urllib.parse.quote(design)}/redact/{urllib.parse.quote(oops)}/m2redact{TXN}", alice, {})

root = send(alice, design, "m.room.message", {"msgtype": "m.text", "body": "Thread: review of the new room list"})
for token, text in ((bob, "Rows look right, badges too."), (alice, "Then I merge it tonight."), (bob, "👍 go")):
    send(token, design, "m.room.message", {
        "msgtype": "m.text", "body": text,
        "m.relates_to": {"rel_type": "m.thread", "event_id": root, "is_falling_back": True,
                          "m.in_reply_to": {"event_id": root}},
    })
print("Seeded reactions, a reply, an edit, a deleted message and a thread in Design")
