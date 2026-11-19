use std::time::Duration;

use cargo_packager_updater::{Config, Update, UpdaterBuilder, semver::Version, url::Url};

// Public half of the signing key. The private half lives in the
// CARGO_PACKAGER_SIGN_PRIVATE_KEY repo secret and signs every release.
const PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDcwRTJGNTA5QkUxRjEwRTQKUldUa0VCKytDZlhpY01kVDZUaGgvVG9XeWZWQkRJVXlRV2tpZ2pjMlpHTzRaclZzc1JjUXoyNWgK";

const ENDPOINT: &str =
  "https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/latest.json";

/// Only builds that can replace themselves get updates. That is any macOS .app, the
/// Windows installer and the `AppImage`. The .deb, the Windows and Linux portables
/// and `cargo run` would be broken by an install, so they skip it.
fn can_self_update() -> bool {
  let Ok(exe) = std::env::current_exe() else {
    return false;
  };
  if cfg!(target_os = "macos") {
    exe.to_string_lossy().contains(".app/Contents/MacOS/")
  } else if cfg!(windows) {
    exe.with_file_name("uninstall.exe").exists()
  } else {
    std::env::var_os("APPIMAGE").is_some()
  }
}

/// Blocking. Returns `None` when there is nothing to install or the check failed.
pub fn check() -> Option<Update> {
  if !can_self_update() {
    return None;
  }
  let config = Config {
    endpoints: vec![Url::parse(ENDPOINT).ok()?],
    pubkey: PUBKEY.to_string(),
    windows: None,
  };
  let current = Version::parse(env!("CARGO_PKG_VERSION")).ok()?;
  UpdaterBuilder::new(current, config)
    .timeout(Duration::from_secs(20))
    .build()
    .ok()?
    .check()
    .ok()
    .flatten()
}

/// Blocking. Downloads, verifies the signature and swaps the app, then starts
/// the new version and exits. On Windows the installer restarts the app itself.
pub fn install(update: &Update) -> Result<(), String> {
  update
    .download_and_install()
    .map_err(|err| err.to_string())?;
  relaunch();
  std::process::exit(0);
}

// The new copy waits a second so this process can let go of the overlay port.
fn relaunch() {
  let target = if cfg!(target_os = "macos") {
    std::env::current_exe()
      .ok()
      .and_then(|exe| exe.ancestors().nth(3).map(std::path::Path::to_path_buf))
      .map(|app| ("open", app.into_os_string()))
  } else {
    std::env::var_os("APPIMAGE").map(|appimage| ("exec", appimage))
  };
  if let Some((cmd, path)) = target {
    let _ = std::process::Command::new("sh")
      .arg("-c")
      .arg(format!("sleep 1; {cmd} \"$0\""))
      .arg(path)
      .spawn();
  }
}

#[cfg(test)]
mod tests {
  use cargo_packager_updater::RemoteRelease;

  // Every target the release workflow ships, with the file the updater must download
  const SHIPPED: [(&str, &str, &str); 6] = [
    (
      "macos-x86_64",
      "pulse-layer-universal-apple-darwin.tar.gz",
      "app",
    ),
    (
      "macos-aarch64",
      "pulse-layer-universal-apple-darwin.tar.gz",
      "app",
    ),
    (
      "windows-x86_64",
      "pulse-layer-x86_64-pc-windows-msvc-setup.exe",
      "nsis",
    ),
    (
      "windows-i686",
      "pulse-layer-i686-pc-windows-msvc-setup.exe",
      "nsis",
    ),
    (
      "windows-aarch64",
      "pulse-layer-aarch64-pc-windows-msvc-setup.exe",
      "nsis",
    ),
    (
      "linux-x86_64",
      "pulse-layer-x86_64-unknown-linux-gnu.AppImage",
      "appimage",
    ),
  ];

  #[test]
  fn manifest_script_matches_what_the_updater_reads() {
    let dist = std::env::temp_dir().join(format!("pulse-layer-manifest-{}", std::process::id()));
    std::fs::create_dir_all(&dist).unwrap();
    for (_, file, _) in SHIPPED {
      std::fs::write(dist.join(format!("{file}.sig")), format!("sig-of-{file}")).unwrap();
    }

    let out = std::process::Command::new("bash")
      .arg(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/scripts/update-manifest.sh"
      ))
      .arg(&dist)
      .args(["owner/repo", "v1.2.3"])
      .output()
      .unwrap();
    std::fs::remove_dir_all(&dist).unwrap();
    assert!(
      out.status.success(),
      "{}",
      String::from_utf8_lossy(&out.stderr)
    );

    let release: RemoteRelease = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(release.version.to_string(), "1.2.3");
    for (target, file, format) in SHIPPED {
      let url = release.download_url(target).unwrap();
      assert_eq!(
        url.as_str(),
        format!("https://github.com/owner/repo/releases/download/v1.2.3/{file}")
      );
      assert_eq!(
        release.signature(target).unwrap(),
        &format!("sig-of-{file}")
      );
      assert_eq!(release.format(target).unwrap().to_string(), format);
    }
  }
}
