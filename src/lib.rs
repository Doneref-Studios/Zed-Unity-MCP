use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

struct UnityMcpExtension;

impl zed::Extension for UnityMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let relay_path = get_relay_path()?;

        Ok(Command {
            command: relay_path,
            args: vec!["--mcp".to_string()],
            env: vec![],
        })
    }
}

/// Resolves the platform-specific path to the Unity MCP relay binary.
///
/// Unity installs the relay to:
/// - Windows: `%USERPROFILE%\.unity\relay\relay_win.exe`
/// - macOS (Apple Silicon): `~/.unity/relay/relay_mac_arm64.app/Contents/MacOS/relay_mac_arm64`
/// - macOS (Intel):         `~/.unity/relay/relay_mac_x64.app/Contents/MacOS/relay_mac_x64`
/// - Linux:                 `~/.unity/relay/relay_linux`
fn get_relay_path() -> Result<String> {
    // Windows: USERPROFILE is always set
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        return Ok(format!(
            "{}\\.unity\\relay\\relay_win.exe",
            userprofile
        ));
    }

    let home = std::env::var("HOME")
        .map_err(|_| "Could not find home directory (HOME / USERPROFILE not set)")?;

    // Prefer arm64 if it exists, otherwise fall back to x64, then Linux
    let mac_arm = format!(
        "{}/.unity/relay/relay_mac_arm64.app/Contents/MacOS/relay_mac_arm64",
        home
    );
    if std::path::Path::new(&mac_arm).exists() {
        return Ok(mac_arm);
    }

    let mac_x64 = format!(
        "{}/.unity/relay/relay_mac_x64.app/Contents/MacOS/relay_mac_x64",
        home
    );
    if std::path::Path::new(&mac_x64).exists() {
        return Ok(mac_x64);
    }

    Ok(format!("{}/.unity/relay/relay_linux", home))
}

zed::register_extension!(UnityMcpExtension);
