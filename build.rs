use std::{env, path::PathBuf, process::Command};

fn main() {
  let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let overlay = root.join("overlay");

  println!("cargo:rerun-if-changed=overlay/index.html");
  println!("cargo:rerun-if-changed=overlay/src");
  println!("cargo:rerun-if-changed=overlay/package.json");
  println!("cargo:rerun-if-changed=overlay/package-lock.json");
  println!("cargo:rerun-if-changed=overlay/tsconfig.json");
  println!("cargo:rerun-if-changed=overlay/vite.config.ts");
  println!("cargo:rerun-if-changed=assets/icons/icon.ico");

  embed_windows_icon();

  // On Windows npm is npm.cmd
  let npm = if cfg!(target_os = "windows") {
    "npm.cmd"
  } else {
    "npm"
  };

  let install = Command::new(npm)
    .arg("ci")
    .current_dir(&overlay)
    .status()
    .expect("npm ci failed — is Node.js installed?");
  assert!(install.success(), "npm ci exited with error");

  let build = Command::new(npm)
    .args(["run", "build"])
    .current_dir(&overlay)
    .status()
    .expect("npm run build failed");
  assert!(build.success(), "Vite build exited with error");
}

#[cfg(windows)]
fn embed_windows_icon() {
  if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
    return;
  }

  let mut resource = winresource::WindowsResource::new();
  resource.set_icon("assets/icons/icon.ico");

  if let Err(error) = resource.compile() {
    println!("cargo:warning=could not embed the Windows icon: {error}");
  }
}

#[cfg(not(windows))]
fn embed_windows_icon() {}
