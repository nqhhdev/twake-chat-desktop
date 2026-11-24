#!/usr/bin/env bash
# Serves the Open Graph test page of link previews on port 18090 (the local Synapse reads
# it as http://host.docker.internal:18090/). Local test content only.
cd "$(dirname "$0")/fixtures/preview" && exec python3 -m http.server 18090 --bind 127.0.0.1
