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
