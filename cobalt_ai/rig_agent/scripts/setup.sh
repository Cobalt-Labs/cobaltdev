#!/bin/bash
echo "=== Rig Agent Setup ==="
if [ -z "$OPENAI_API_KEY" ] && [ -z "$GEMINI_API_KEY" ] && [ -z "$ANTHROPIC_API_KEY" ] && [ -z "$COHERE_API_KEY" ]; then
    echo "Warning: No LLM API keys found in your environment."
    echo "You can set them in a .env file inside the rig_agent directory."
else
    echo "Success: Found active API keys in your environment!"
fi
cargo build
