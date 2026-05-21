# Tauri + Vanilla
## AI Color Palette Generator

Run this command to compile after installing rust and tauri
```
cargo tauri dev
```

## Configuration

The LM Studio API endpoint is configurable via environment variable:

| Variable | Default | Description |
|---|---|---|
| `LMSTUDIO_API_URL` | `http://localhost:1234/v1/chat/completions` | Full URL to the LM Studio chat completions endpoint |
| `LMSTUDIO_MODEL` | `qwen/qwen3.6-27b` | Model name to use for palette generation |

Example for a remote LM Studio instance with a custom model:
```bash
LMSTUDIO_API_URL="http://192.168.50.61:1234/v1/chat/completions" \
LMSTUDIO_MODEL="qwen/qwen3.6-27b" \
cargo tauri dev
```