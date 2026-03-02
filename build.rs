use std::{env, path::PathBuf, process::Command};

fn main() {
  let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let overlay = root.join("overlay");

  println!("cargo:rerun-if-changed=overlay/index.html");
  println!("cargo:rerun-if-changed=overlay/src/main.ts");
  println!("cargo:rerun-if-changed=overlay/src/style.css");
  println!("cargo:rerun-if-changed=overlay/vite.config.js");

  // On Windows npm is npm.cmd
  let npm = if cfg!(target_os = "windows") {
    "npm.cmd"
  } else {
    "npm"
  };

  let install = Command::new(npm)
    .args(["install", "--prefer-offline"])
    .current_dir(&overlay)
    .status()
    .expect("npm install failed — is Node.js installed?");
  assert!(install.success(), "npm install exited with error");

  let build = Command::new(npm)
    .args(["run", "build"])
    .current_dir(&overlay)
    .status()
    .expect("npm run build failed");
  assert!(build.success(), "Vite build exited with error");
}
