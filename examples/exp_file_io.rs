//! EXPERIMENT: file I/O with the standard library (native only).
//!
//! Run:  cargo run --example exp_file_io
//!
//! Everything happens inside a scratch folder in the system temp directory,
//! which is deleted at the end.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::path::{Path, PathBuf};

// `io::Result<T>` is shorthand for `Result<T, io::Error>`.
// Because `main` returns it, we can use `?` for every file operation: the first
// failure ends the program and prints the error.
fn main() -> io::Result<()> {
    // -- 1. paths ---------------------------------------------------------------------
    // `PathBuf` is an owned path (like String), `&Path` a borrowed one (like &str).
    // `join` adds a component with the right separator for the OS.
    let dir: PathBuf = std::env::temp_dir().join(format!("foxg-exp-{}", std::process::id()));
    let file: PathBuf = dir.join("notes.txt");
    println!("1. file = {}", file.display()); // `display()` makes a path printable

    // -- 2. create a folder, write a file ------------------------------------------------
    fs::create_dir_all(&dir)?; // like `mkdir -p`: fine if it already exists
    fs::write(&file, "fox\nowl\n")?; // create or REPLACE the whole file
    println!("2. wrote {} bytes", fs::metadata(&file)?.len());

    // -- 3. read it back ----------------------------------------------------------------------
    let text = fs::read_to_string(&file)?; // whole file into one String
    println!("3. read {:?}", text);

    // -- 4. append, with OpenOptions ---------------------------------------------------------------
    // `OpenOptions` is a builder: describe HOW to open, then `open`.
    let mut handle = OpenOptions::new().append(true).open(&file)?;
    writeln!(handle, "hare")?; // `writeln!` writes formatted text to anything that implements Write
    // `handle` is dropped at the end of main (or the scope): the file is closed automatically.
    drop(handle);

    // -- 5. read line by line (good for big files) ------------------------------------------------------
    let reader = BufReader::new(File::open(&file)?); // buffered: fewer slow disk reads
    for (number, line) in reader.lines().enumerate() {
        println!("5. line {}: {}", number + 1, line?); // each line is an io::Result<String>
    }

    // -- 6. errors are values ------------------------------------------------------------------------------
    let missing: &Path = &dir.join("does-not-exist.txt");
    match fs::read_to_string(missing) {
        Ok(_) => println!("6. (unexpected) it exists"),
        // Look at WHAT KIND of error: not found is different from "permission denied".
        Err(e) if e.kind() == ErrorKind::NotFound => println!("6. not found: that is fine, treat as 'never saved'"),
        Err(e) => return Err(e), // anything else: pass it up
    }

    // -- 7. the safe-replace trick used by the app ----------------------------------------------------------
    // Write the new content to a temporary file, then RENAME it over the old one.
    // A rename is atomic on normal file systems: readers see the complete old file or
    // the complete new file, never half of a write.
    let temp = dir.join("notes.txt.tmp");
    fs::write(&temp, "replaced\n")?;
    fs::rename(&temp, &file)?;
    println!("7. after safe replace: {:?}", fs::read_to_string(&file)?);

    // -- 8. clean up -------------------------------------------------------------------------------------------
    fs::remove_dir_all(&dir)?;
    println!("8. removed {}; exists now? {}", dir.display(), dir.exists());

    // Things to try:
    //   - make `dir` a path you cannot write to (e.g. "C:\\Windows\\x" or "/proc/x") and read the error text;
    //   - remove the `?` after create_dir_all and read the "unused Result" warning.
    Ok(())
}
