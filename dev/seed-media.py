#!/usr/bin/env python3
"""Seeds media on the local Synapse (dev/up.sh, dev/seed.sh first): avatars for alice and
bob, a picture for the room General, a photo and a file posted in General.
Local test values only. Images are drawn here, no download."""
import json
import struct
import urllib.request
import zlib

HS = "http://localhost:18008"


def png(width, height, pixel):
    rows = b"".join(
        b"\x00" + b"".join(bytes(pixel(x, y)) for x in range(width)) for y in range(height)
    )
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    header = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(rows, 9)) + chunk(b"IEND", b"")


def call(method, path, token=None, body=None, raw=None, content_type="application/json"):
    data = raw if raw is not None else (json.dumps(body).encode() if body is not None else None)
    request = urllib.request.Request(HS + path, data=data, method=method)
    request.add_header("Content-Type", content_type)
    if token:
        request.add_header("Authorization", f"Bearer {token}")
    with urllib.request.urlopen(request) as response:
        return json.loads(response.read() or b"{}")


def login(user):
    return call("POST", "/_matrix/client/v3/login", body={
        "type": "m.login.password",
        "identifier": {"type": "m.id.user", "user": user},
        "password": f"{user}-dev-password",
    })["access_token"]


def upload(token, data, name, mime):
    return call("POST", f"/_matrix/media/v3/upload?filename={name}", token, raw=data, content_type=mime)["content_uri"]


def room_named(token, name):
    for room_id in call("GET", "/_matrix/client/v3/joined_rooms", token)["joined_rooms"]:
        try:
            state = call("GET", f"/_matrix/client/v3/rooms/{room_id}/state/m.room.name/", token)
        except urllib.error.HTTPError:
            continue
        if state.get("name") == name:
            return room_id
    raise SystemExit(f"no room named {name}: run dev/seed.sh first")


alice, bob = login("alice"), login("bob")
alice_avatar = upload(alice, png(128, 128, lambda x, y: (40 + x, 120, 220 - y)), "alice.png", "image/png")
bob_avatar = upload(bob, png(128, 128, lambda x, y: (230, 90 + y // 2, 40 + x // 2)), "bob.png", "image/png")
call("PUT", "/_matrix/client/v3/profile/@alice:localhost/avatar_url", alice, {"avatar_url": alice_avatar})
call("PUT", "/_matrix/client/v3/profile/@bob:localhost/avatar_url", bob, {"avatar_url": bob_avatar})

general = room_named(alice, "General")
room_picture = upload(alice, png(128, 128, lambda x, y: (120 + (x ^ y) % 100, 60, 200)), "general.png", "image/png")
call("PUT", f"/_matrix/client/v3/rooms/{general}/state/m.room.avatar/", alice, {"url": room_picture})

photo = png(480, 320, lambda x, y: ((x * 255) // 480, (y * 255) // 320, 160))
photo_uri = upload(bob, photo, "sunset.png", "image/png")
call("PUT", f"/_matrix/client/v3/rooms/{general}/send/m.room.message/media1", bob, {
    "msgtype": "m.image", "body": "sunset.png", "url": photo_uri,
    "info": {"mimetype": "image/png", "w": 480, "h": 320, "size": len(photo)},
})
report = b"%PDF-1.4\n% Twake desktop test file\n" + b"0" * 20000
report_uri = upload(alice, report, "release-notes.pdf", "application/pdf")
call("PUT", f"/_matrix/client/v3/rooms/{general}/send/m.room.message/media2", alice, {
    "msgtype": "m.file", "body": "release-notes.pdf", "url": report_uri,
    "info": {"mimetype": "application/pdf", "size": len(report)},
})
call("PUT", f"/_matrix/client/v3/rooms/{general}/send/m.room.message/media3", bob, {
    "msgtype": "m.text", "body": "Pictures and files should show above.",
})
print("Seeded avatars, a room picture, a photo and a file in General")
