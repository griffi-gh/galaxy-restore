use std::{
  path::{Path, PathBuf},
  sync::OnceLock,
};

const POSSIBLE_SDB_PATHS: &[&str] = &[
  "tizen-studio/tools/sdb",
  ".tizen-studio/tools/sdb",
  "tizen-sdk/tools/sdb",
  ".tizen/tizen-studio/tools/sdb",
  ".tizen/.tizen-studio/tools/sdb",
  ".tizen/tizen-sdk/tools/sdb",
];

fn discover_sdb() -> Option<PathBuf> {
  tracing::debug!("Checking SDB_PATH");

  // 0. Check for SDB_PATH env var
  if let Ok(sdb_path) = std::env::var("SDB_PATH") {
    if let Ok(canonical_path) = PathBuf::from(sdb_path).canonicalize() {
      if std::fs::exists(&canonical_path).unwrap_or(false) {
        tracing::info!("Found sdb at SDB_PATH: {}", canonical_path.display());
        return Some(canonical_path);
      }
    }
  }

  // 1. Try PATH
  tracing::debug!("Checking PATH");
  if let Some(path) = std::env::var_os("PATH") {
    for path in std::env::split_paths(&path) {
      let sdb_path = path.join("sdb");
      if std::fs::exists(&sdb_path).unwrap_or(false) {
        tracing::info!("Found sdb in PATH");
        return Some(sdb_path);
      }
    }
  }

  // 2. Try, paths in POSSIBLE_SDB_PATHS
  if let Some(home) = dirs::home_dir() {
    for &path in POSSIBLE_SDB_PATHS.iter() {
      tracing::debug!("Checking path: {}", path);
      let path = PathBuf::from(path);
      if let Ok(canonical_path) = home.join(&path).canonicalize() {
        if std::fs::exists(&canonical_path).unwrap_or(false) {
          tracing::info!("Found sdb at {}", path.display());
          return Some(canonical_path);
        }
      }
    }
  }

  tracing::warn!("Failed to find sdb");

  None
}

pub fn sdb_path() -> &'static Path {
  static SDB_PATH: OnceLock<PathBuf> = OnceLock::new();
  SDB_PATH.get_or_init(|| discover_sdb().expect("failed to find sdb"))
}
