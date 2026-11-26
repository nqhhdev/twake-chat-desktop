#!/usr/bin/env python3
"""Seeds M5 cases (dev/seed.sh and the earlier seeds first): every member may join the
calls of Design (clear) and Secret project (encrypted), as in the rooms the app creates
(the call membership state event at power level 0, web `powerLevelsOverride`).
Local test values only."""
import json
import urllib.parse
import urllib.request

HS = "http://localhost:18008"
CALL_MEMBER_TYPE = "org.matrix.msc3401.call.member"


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
        "password": f"{user}-dev-password", "device_id": "SEEDM5"})["access_token"]


def room_named(token, name):
    for room_id in call("GET", "/_matrix/client/v3/joined_rooms", token)["joined_rooms"]:
        try:
            state = call("GET", f"/_matrix/client/v3/rooms/{urllib.parse.quote(room_id)}/state/m.room.name/", token)
        except urllib.error.HTTPError:
            continue
        if state.get("name") == name:
            return room_id
    raise SystemExit(f"no room named {name}")


tokens = {user: login(user) for user in ["alice", "bob"]}
for name in ["Design", "Secret project"]:
    room = urllib.parse.quote(room_named(tokens["alice"], name))
    levels = call("GET", f"/_matrix/client/v3/rooms/{room}/state/m.room.power_levels/", tokens["alice"])
    levels.setdefault("events", {})[CALL_MEMBER_TYPE] = 0
    # Changed by whichever test user may (a creator holds power without being listed)
    for token in tokens.values():
        try:
            call("PUT", f"/_matrix/client/v3/rooms/{room}/state/m.room.power_levels/", token, levels)
            break
        except urllib.error.HTTPError:
            continue
    else:
        raise SystemExit(f"{name}: no test user may change its power levels")
    print(f"{name}: calls open to every member")
for token in tokens.values():
    call("POST", "/_matrix/client/v3/logout", token, {})
