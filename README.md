# Zed Unity MCP

A [Zed](https://zed.dev) extension that connects Zed's Agent Panel to the [Unity Editor](https://unity.com) via Unity's official MCP relay.

## Prerequisites

- **Unity 6** (6000.0) or later with the `com.unity.ai.assistant` package installed
- The **Unity MCP relay** binary — automatically installed to `~/.unity/relay/` when the Unity Editor starts

## Setup

1. Open your Unity project in the Editor. Go to **Edit → Project Settings → AI → Unity MCP** and confirm the **Unity Bridge** status shows **Running**.

2. Install this extension in Zed from the Extensions marketplace (`unity-mcp`).

3. Enable context server tools in your Zed agent profile. Add the following to your Zed `settings.json` (Cmd/Ctrl+Shift+P → `zed: open settings`):

```json
"agent": {
  "profiles": {
    "write": { "name": "Write", "enable_all_context_servers": true },
    "ask":   { "name": "Ask",   "enable_all_context_servers": true }
  }
}
```

> Without this, Zed silently disables all MCP context server tools in every profile by default.

4. Start the Agent Panel in Zed and the relay will be launched automatically.

5. On first connection, Unity will show a **Pending Connection** prompt — click **Accept** to authorize Zed.

## How It Works

The extension launches Unity's relay binary with the `--mcp` flag, which starts an MCP server that bridges Zed's Agent Panel to the live Unity Editor session.

| Platform | Relay Path |
|----------|-----------|
| Windows | `%USERPROFILE%\.unity\relay\relay_win.exe` |
| macOS (Apple Silicon) | `~/.unity/relay/relay_mac_arm64.app/…/relay_mac_arm64` |
| macOS (Intel) | `~/.unity/relay/relay_mac_x64.app/…/relay_mac_x64` |
| Linux | `~/.unity/relay/relay_linux` |

## Resources

- [Unity MCP – Get Started](https://docs.unity3d.com/Packages/com.unity.ai.assistant@2.0/manual/unity-mcp-get-started.html)
- [Zed MCP Extensions](https://zed.dev/docs/extensions/mcp-extensions)
- [Zed Agent Panel](https://zed.dev/docs/ai/mcp)

## License

MIT — see [LICENSE](LICENSE)
