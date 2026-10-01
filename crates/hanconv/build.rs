use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");

    println!("cargo:rerun-if-changed={}", data_dir.display());

    for entry in fs::read_dir(data_dir)? {
        let path = entry?.path();

        if path.extension().is_some_and(|extension| extension == "txt")
            && fs::read_to_string(path)?
                .chars()
                .any(|c| !c.is_ascii() && c.is_whitespace())
        {
            return Err("a dictionary contains non-ASCII whitespace".into());
        }
    }

    Ok(())
}
