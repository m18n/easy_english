use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
//test
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();
    let source_folder = Path::new("easy_english_web");

    let dockerfile_source = Path::new("Dockerfile");
    let dockerfile_dest =target_dir.join("Dockerfile");

    fs::copy(dockerfile_source, dockerfile_dest).expect("Failed to copy Dockerfile");

    // Copy .dockerignore
    let dockerignore_source = Path::new(".dockerignore");
    let dockerignore_dest =target_dir.join(".dockerignore");
    let env = Path::new(".env");
    let env_dest =target_dir.join(".env");
    fs::copy(env, env_dest).expect("Failed to copy .env");

    let new_path = target_dir.join("easy_english_web");
    if new_path.exists() {
        fs::remove_dir_all(&new_path).expect("Failed to remove old new_folder_name");
    }
    DirBuilder::new().recursive(true).create(&new_path).expect("Failed to create new_folder_name directory");

    for entry in WalkDir::new(source_folder).into_iter().filter_map(|e| e.ok()) {
        let rel_path = entry.path().strip_prefix(source_folder).unwrap();
        let dest_path = new_path.join(rel_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(dest_path).expect("Failed to create directory in new_folder_name");
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directory");
            }
            fs::copy(entry.path(), &dest_path).expect("Failed to copy file to new_folder_name");
        }
    }
}