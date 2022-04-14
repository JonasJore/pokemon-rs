use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

const SOURCE_DIR: &str = "./src/data";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/lib.rs");
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("data.rs");
    let mut files_to_be_added = File::create(&dest_path)?;

    writeln!(&mut files_to_be_added, r##"["##,)?;

    for f in fs::read_dir(SOURCE_DIR)? {
        let f = f?;

        if !f.file_type()?.is_file() {
            continue;
        }

        writeln!(
            &mut files_to_be_added,
            r##"("{name}", include_bytes!(r#"{name}"#)),"##,
            name = f.path().display(),
        )?;
    }

    writeln!(&mut files_to_be_added, r##"]"##,)?;

    Ok(())
}