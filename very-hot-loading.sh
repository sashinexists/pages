
#!/bin/bash

# Folder to watch
WATCH_FOLDER="./src"

# Commands to run
BUILD_COMMAND="cargo run"
OPEN_COMMAND="flatpak run org.mozilla.firefox --no-remote -P dev .public/index.html"

# Counter for loop iterations
count=0

while inotifywait -r -e modify,create,delete,move "$WATCH_FOLDER"; do
    # Increment the counter
    ((count++))

    echo "Folder changes detected, running cargo... (Iteration: $count)"
    $BUILD_COMMAND || echo "Failed to run cargo."
    
    # Check if Firefox with dev profile is already running
    WID=$(xdotool search --name "dev")

    if [ $? -ne 0 ]; then
        echo "Failed to find window ID."
    fi

    if [ -z "$WID" ]; then
        echo "Opening .public/index.html with dev profile..."
        $OPEN_COMMAND || echo "Failed to open Firefox."
    else
        echo "Refreshing .public/index.html..."
        xdotool windowactivate --sync $WID key F5 || echo "Failed to refresh Firefox."
    fi

    echo "Loop iteration complete. (Total Iterations: $count)"
done
