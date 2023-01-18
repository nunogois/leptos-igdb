#!/bin/bash
PATH=$PATH:/vercel/.cargo/bin

# cargo leptos build --release
npx tailwindcss -i ./style/main.css -o ./style/tailwind.css --minify
trunk build --release