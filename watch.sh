#!/bin/bash

cargo watch -s "wasm-pack build --target web && echo 'Build complete. Refresh your browser.'"