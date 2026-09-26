use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");

    println!("cargo:rerun-if-changed={}", data_dir.display());

    for file in dictionary_files(&data_dir) {
        println!("cargo:rerun-if-changed={}", file.display());

        if contains_non_ascii_whitespace(&read_dictionary(&file)) {
            panic!("a dictionary contains non-ASCII whitespace");
        }
    }
}

fn dictionary_files(data_dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(data_dir)
        .unwrap_or_else(|_| panic!("failed to read the data directory"))
        .map(|entry| entry.expect("failed to read a directory entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "txt"))
        .collect();

    files.sort();

    files
}

fn read_dictionary(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read a dictionary file"))
}

fn contains_non_ascii_whitespace(text: &str) -> bool {
    text.chars()
        .any(|character| !character.is_ascii() && character.is_whitespace())
}
