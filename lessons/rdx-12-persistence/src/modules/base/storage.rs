//! Saving small pieces of text, on every platform, with the standard library.
//!
//! THE TWO WORLDS (from lesson 01)
//!
//! ```text
//!   desktop / Android / iOS : our Rust code is NATIVE -> `std::fs` works
//!   web                     : our Rust code is WebAssembly in a sandbox
//!                             -> no files; the browser offers `localStorage`
//! ```
//!
//! One public API, two private implementations, chosen AT COMPILE TIME with
//! `#[cfg(...)]`. Code for the other platform is not even compiled:
//!
//! ```text
//!   save("progress", text) --+-- wasm32 build ----> web::save   (JavaScript localStorage)
//!                            `-- native build -----> native::save (a file on disk)
//! ```
//!
//! WHAT IS SAVED: plain text (`String`). Turning a struct into text and back is
//! the job of the `Persist` trait below. No serialization crate: you will see
//! exactly what a "file format" is (see `modules/games/progress.rs`).

use std::fmt;

// Only the web implementation talks to JavaScript, so only the web build needs this import.
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Everything that can go wrong while reading or writing.
///
/// One enum for all causes, so callers have ONE error type to handle, whatever
/// platform they run on. (Same recipe as `GuessError` in lesson 06.)
#[derive(Debug)]
pub enum StorageError {
    /// A file operation failed (native only). Wraps the original `io::Error`.
    #[cfg(not(target_arch = "wasm32"))]
    Io(std::io::Error),
    /// The JavaScript side failed (web only).
    #[cfg(target_arch = "wasm32")]
    Script(String),
    /// The saved text could not be understood.
    Parse(ParseError),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            StorageError::Io(e) => write!(f, "file error: {e}"),
            #[cfg(target_arch = "wasm32")]
            StorageError::Script(message) => write!(f, "browser storage error: {message}"),
            StorageError::Parse(e) => write!(f, "saved data is damaged: {e}"),
        }
    }
}

impl std::error::Error for StorageError {}

// Lets `?` turn an `io::Error` into a `StorageError` automatically.
#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> StorageError {
        StorageError::Io(e)
    }
}

impl From<ParseError> for StorageError {
    fn from(e: ParseError) -> StorageError {
        StorageError::Parse(e)
    }
}

/// Why saved text could not be turned back into a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// A line without `=`. Carries the line.
    BadLine(String),
    /// A value that is not a number. Carries the key and the text.
    BadNumber { key: String, value: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::BadLine(line) => write!(f, "line '{line}' has no '='"),
            ParseError::BadNumber { key, value } => write!(f, "'{value}' is not a number (key '{key}')"),
        }
    }
}

impl std::error::Error for ParseError {}

// ---------------------------------------------------------------------------
// The Persist trait: value <-> text
// ---------------------------------------------------------------------------

/// A type that can be written as text and read back.
///
/// WHY A TRAIT? `save_value` and `load_value` below work for ANY such type
/// (progress today, settings tomorrow) and you write the file logic only once.
///
/// `Sized`: the value must have a known size, because `from_text` returns it by value.
pub trait Persist: Sized {
    fn to_text(&self) -> String;
    fn from_text(text: &str) -> Result<Self, ParseError>;
}

/// Save any `Persist` value under `name`.
pub async fn save_value<T: Persist>(name: &str, value: &T) -> Result<(), StorageError> {
    save(name, &value.to_text()).await
}

/// Load a value. `Ok(None)` = nothing saved yet (a normal first start, NOT an error).
pub async fn load_value<T: Persist>(name: &str) -> Result<Option<T>, StorageError> {
    match load(name).await? {
        Some(text) => Ok(Some(T::from_text(&text)?)), // two `?`: storage error, then parse error
        None => Ok(None),
    }
}

// ---------------------------------------------------------------------------
// The platform switch
// ---------------------------------------------------------------------------

/// Read the text saved under `name`. `Ok(None)` if there is none.
pub async fn load(name: &str) -> Result<Option<String>, StorageError> {
    #[cfg(target_arch = "wasm32")]
    {
        web::load(name).await
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::load_from(&native::data_dir(), name)
    }
}

/// Write `text` under `name`, replacing what was there.
pub async fn save(name: &str, text: &str) -> Result<(), StorageError> {
    #[cfg(target_arch = "wasm32")]
    {
        web::save(name, text).await
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::save_to(&native::data_dir(), name, text)
    }
}

/// A human-readable "where is my data?", for the UI.
pub fn location_hint() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        String::from("browser storage (localStorage)")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::data_dir().display().to_string()
    }
}

// ---------------------------------------------------------------------------
// Native implementation: plain files
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::fs;
    use std::io::ErrorKind;
    use std::path::{Path, PathBuf};

    use super::StorageError;

    /// Where the app keeps its files. First rule that applies wins:
    ///
    ///   1. `FOXG_DATA_DIR` (you can point the app anywhere)
    ///   2. Windows: `%APPDATA%\foxg-hub`
    ///   3. macOS / Linux: `$HOME/.local/share/foxg-hub`
    ///   4. fallback: the system temp folder (Android has none of the above for us;
    ///      temp data may be deleted by the system, a known limit of "std only")
    ///
    /// `var_os` returns `Option<OsString>`: `None` when the variable does not exist.
    /// `Path`/`PathBuf` are the std types for file paths: `Path` is the borrowed form
    /// (like `&str`), `PathBuf` the owned form (like `String`).
    pub fn data_dir() -> PathBuf {
        if let Some(dir) = std::env::var_os("FOXG_DATA_DIR") {
            return PathBuf::from(dir);
        }
        if let Some(appdata) = std::env::var_os("APPDATA") {
            return PathBuf::from(appdata).join("foxg-hub");
        }
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(".local").join("share").join("foxg-hub");
        }
        std::env::temp_dir().join("foxg-hub")
    }

    /// The file for `name` inside `dir`. `name.txt`.
    fn file_for(dir: &Path, name: &str) -> PathBuf {
        dir.join(format!("{name}.txt"))
    }

    /// Read a file. A missing file is NOT an error here: it just means "never saved".
    pub fn load_from(dir: &Path, name: &str) -> Result<Option<String>, StorageError> {
        match fs::read_to_string(file_for(dir, name)) {
            Ok(text) => Ok(Some(text)),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None), // match guard on the error kind
            Err(e) => Err(StorageError::Io(e)),
        }
    }

    /// Write a file, creating the folder first if needed.
    ///
    /// SAFE REPLACE: write to `name.txt.tmp`, then rename over the real file. If the
    /// program dies in the middle of the write, the old file is still complete.
    pub fn save_to(dir: &Path, name: &str, text: &str) -> Result<(), StorageError> {
        fs::create_dir_all(dir)?; // `?` converts io::Error -> StorageError via From
        let target = file_for(dir, name);
        let temp = dir.join(format!("{name}.txt.tmp"));
        fs::write(&temp, text)?;
        fs::rename(&temp, &target)?;
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// A fresh empty folder for one test. Tests run in parallel, so every test
        /// needs its OWN folder name.
        fn test_dir(label: &str) -> PathBuf {
            let dir = std::env::temp_dir().join(format!("foxg-test-{}-{label}", std::process::id()));
            let _ = fs::remove_dir_all(&dir); // leftovers from a crashed run: ignore errors
            dir
        }

        #[test]
        fn missing_file_is_none_not_an_error() {
            let dir = test_dir("missing");
            assert!(matches!(load_from(&dir, "nothing"), Ok(None)));
        }

        #[test]
        fn save_then_load_returns_the_same_text() {
            let dir = test_dir("roundtrip");
            save_to(&dir, "progress", "a=1\nb=2\n").unwrap();
            assert_eq!(load_from(&dir, "progress").unwrap().as_deref(), Some("a=1\nb=2\n"));
            fs::remove_dir_all(&dir).unwrap();
        }

        #[test]
        fn saving_again_replaces_the_text() {
            let dir = test_dir("replace");
            save_to(&dir, "x", "old").unwrap();
            save_to(&dir, "x", "new").unwrap();
            assert_eq!(load_from(&dir, "x").unwrap().as_deref(), Some("new"));
            fs::remove_dir_all(&dir).unwrap();
        }

        #[test]
        fn no_temp_file_is_left_behind() {
            let dir = test_dir("tmp");
            save_to(&dir, "x", "data").unwrap();
            assert!(!dir.join("x.txt.tmp").exists());
            fs::remove_dir_all(&dir).unwrap();
        }
    }
}

// ---------------------------------------------------------------------------
// Web implementation: localStorage through JavaScript
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web {
    use super::{document, StorageError};

    // We never paste `name` or `text` into the script. They travel through
    // `eval.send(...)` and the script reads them with `dioxus.recv()`. Pasting
    // user text into code is how injection bugs happen.

    pub async fn load(name: &str) -> Result<Option<String>, StorageError> {
        let eval = document::eval(
            "const key = await dioxus.recv(); return window.localStorage.getItem('foxg:' + key);",
        );
        eval.send(name).map_err(|e| StorageError::Script(e.to_string()))?;
        eval.join::<Option<String>>().await.map_err(|e| StorageError::Script(e.to_string()))
    }

    pub async fn save(name: &str, text: &str) -> Result<(), StorageError> {
        let eval = document::eval(
            "const key = await dioxus.recv(); const value = await dioxus.recv();
             window.localStorage.setItem('foxg:' + key, value); return true;",
        );
        eval.send(name).map_err(|e| StorageError::Script(e.to_string()))?;
        eval.send(text).map_err(|e| StorageError::Script(e.to_string()))?;
        eval.join::<bool>().await.map_err(|e| StorageError::Script(e.to_string()))?;
        Ok(())
    }
}
