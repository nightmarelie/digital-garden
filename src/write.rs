use std::io::{Read, Write};
use std::path::PathBuf;

use color_eyre::eyre::{Result, WrapErr};
use edit::{Builder, edit_file};

const TEMPLATE: &[u8; 2] = b"# ";


pub fn write(garden_path: PathBuf, _title: Option<String>) -> Result<()> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create wip file")?
        .keep()
        .wrap_err("Failed to keep tempfile")?;
    file.write_all(TEMPLATE)?;
    // let the user write whatever they want in their favorite editor
    // before returning to the cli and finishing up
    edit_file(filepath)?;
    // Read the user's changes back from the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    dbg!(contents);
    todo!()
}