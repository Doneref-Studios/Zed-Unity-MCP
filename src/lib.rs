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

/// Derives the user's home directory from the `PWD` env var that Zed injects into
/// the WASM sandbox.
///
/// Zed sets `PWD` to the extension work directory (e.g.
/// `C:/Users/Alice/AppData/Roaming/Zed/extensions/work/unity-mcp` on Windows,
/// `/Users/Alice/Library/.../work/unity-mcp` on macOS,
/// `/home/alice/.config/.../work/unity-mcp` on Linux).
/// We parse the username from that path and reconstruct the home dir.
fn home_from_pwd() -> Option<String> {
    let pwd = std::env::var("PWD").ok()?;
    let (os, _) = zed::current_platform();
    let parts: Vec<&str> = pwd.split('/').collect();

    match os {
        // Windows PWD: "C:/Users/Username/..."  → parts = ["C:", "Users", "Username", ...]
        zed::Os::Windows => {
            if parts.len() >= 3 && parts[1].eq_ignore_ascii_case("Users") {
                Some(format!("{}/Users/{}", parts[0], parts[2]))
            } else {
                None
            }
        }
        // macOS PWD: "/Users/Username/..."  → parts = ["", "Users", "Username", ...]
        zed::Os::Mac => {
            if parts.len() >= 3 && parts[1] == "Users" {
                Some(format!("/Users/{}", parts[2]))
            } else {
                None
            }
        }
        // Linux PWD: "/home/username/..."  → parts = ["", "home", "username", ...]
        zed::Os::Linux => {
            if parts.len() >= 3 && parts[1] == "home" {
                Some(format!("/home/{}", parts[2]))
            } else {
                None
            }
        }
    }
}

/// Resolves the platform-specific path to the Unity MCP relay binary.
///
/// Unity installs the relay to:
/// - Windows:              `%USERPROFILE%\.unity\relay\relay_win.exe`
/// - macOS (Apple Silicon): `~/.unity/relay/relay_mac_arm64.app/Contents/MacOS/relay_mac_arm64`
/// - macOS (Intel):         `~/.unity/relay/relay_mac_x64.app/Contents/MacOS/relay_mac_x64`
/// - Linux:                 `~/.unity/relay/relay_linux`
fn get_relay_path() -> Result<String> {
    let home = home_from_pwd()
        .ok_or("Could not determine home directory: PWD env var missing or has unexpected format")?;
    let (os, arch) = zed::current_platform();

    let path = match (os, arch) {
        (zed::Os::Windows, _) => {
            // Convert forward-slash PWD form back to Windows backslashes.
            format!("{}\\.unity\\relay\\relay_win.exe", home.replace('/', "\\"))
        }
        (zed::Os::Mac, zed::Architecture::Aarch64) => {
            format!(
                "{}/.unity/relay/relay_mac_arm64.app/Contents/MacOS/relay_mac_arm64",
                home
            )
        }
        (zed::Os::Mac, _) => {
            format!(
                "{}/.unity/relay/relay_mac_x64.app/Contents/MacOS/relay_mac_x64",
                home
            )
        }
        (zed::Os::Linux, _) => {
            format!("{}/.unity/relay/relay_linux", home)
        }
    };

    Ok(path)
}

zed::register_extension!(UnityMcpExtension);
