#!/bin/bash

# Replace with the folder you want to watch
WATCH_FOLDER="src"

# Replace with the command you want to execute when a change is detected
COMMAND="cargo run"

while inotifywait -r -e modify,create,delete,move "$WATCH_FOLDER"; do
    echo "Folder changes detected, running command..."
    $COMMAND
done
