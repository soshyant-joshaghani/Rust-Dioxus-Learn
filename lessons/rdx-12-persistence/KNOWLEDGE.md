# Lesson 12 - KNOWLEDGE (the book part)

Read with `src/modules/base/storage.rs`, `src/modules/games/progress.rs`, and `examples/exp_file_io.rs`.

---

## 1. Why persistence is a *design* problem

Saving looks like "write a file". In practice you must decide:

| Question | Our answer |
|----------|-----------|
| Where does the data live? | desktop: a file in the user's app-data folder; web: `localStorage` |
| What format? | plain text `key=value` lines you can read in Notepad |
| What if it was never saved? | `Ok(None)`: a normal first start |
| What if it is damaged? | an error value, shown to the player; never silently thrown away |
| What if an old/new version wrote it? | tolerate missing and unknown keys |
| What if the program dies while writing? | write a temp file, then rename |
| Who is allowed to change it? | only `ProgressStore::update`, which saves after every change |

## 2. Why `Result<T, E>` and not just `T`

`load` could return a `String` and "" for failure. That would make three different situations look identical:

1. the file does not exist (fine),
2. the file exists but cannot be read (permissions, disk error),
3. the file is empty because the player really has no data.

With `Result<Option<String>, StorageError>` they are three different values: `Ok(None)`, `Err(...)`, `Ok(Some(""))`. The type forces every caller to decide what each one means. **When a function talks to the outside world (disk, network, user input), it can fail, so it returns `Result`.** A pure calculation (`judge`) cannot, so it returns the value.

In `load_from`:

```rust
match fs::read_to_string(path) {
    Ok(text) => Ok(Some(text)),
    Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),   // expected: first run
    Err(e) => Err(StorageError::Io(e)),                      // unexpected: report it
}
```

The match **guard** turns one specific error into a normal outcome. Every other error is passed up.

## 3. `std::fs` essentials

| Call | Does |
|------|------|
| `fs::read_to_string(path)` | whole file -> `String` |
| `fs::write(path, text)` | create or replace the file |
| `fs::create_dir_all(dir)` | make a folder and parents |
| `fs::rename(from, to)` | move / replace atomically |
| `fs::remove_file`, `remove_dir_all` | delete |
| `File::open` + `BufReader::lines()` | read big files line by line |
| `OpenOptions::new().append(true).open(..)` | add to the end |

Everything returns `io::Result<T>`. A file is closed when its handle is **dropped** (ownership again: no `close()` call to forget).

### Paths

`PathBuf` (owned, like `String`) and `&Path` (borrowed, like `&str`). Build them with `.join(...)`, never with `format!("{}/{}")`: the separator differs per OS. `path.display()` prints one.

### Environment variables

`std::env::var_os("NAME")` returns `Option<OsString>`. `None` when missing. `data_dir()` is a chain of `if let Some(...)` checks: the first match wins. It reads like a priority list because it *is* one.

## 4. The atomic-replace trick

```rust
fs::write(&temp, text)?;      // 1. write the new content next to the old file
fs::rename(&temp, &target)?;  // 2. swap: one step, all or nothing
```

If the app crashes during step 1, `progress.txt` is untouched. If it crashes during step 2, either the old or the new file is there, complete. Writing directly into the real file would leave a half-written (corrupt) file after a crash. The test `no_temp_file_is_left_behind` checks the cleanup.

## 5. Two platforms, one API: `cfg`

```rust
pub async fn load(name: &str) -> Result<Option<String>, StorageError> {
    #[cfg(target_arch = "wasm32")]
    { web::load(name).await }
    #[cfg(not(target_arch = "wasm32"))]
    { native::load_from(&native::data_dir(), name) }
}
```

- `#[cfg(...)]` includes or **removes** the next item **at compile time**. On a native build the `web` module is not compiled at all; on a web build, `native` is not.
- That is why `StorageError` has `Io(..)` only for native and `Script(..)` only for web: the variant that cannot occur on a platform does not exist there.
- The public functions (`load`, `save`) have the **same signature** everywhere, so callers (the store) have no `cfg` at all. **Put platform differences behind one small interface and keep them out of the rest of the code.**
- `cfg!(...)` (with a bang) is an ordinary `bool` expression; `#[cfg(...)]` removes code. Use the attribute when the code would not even compile on the other platform.
- Test both branches: `cargo check` (native) and `cargo check --target wasm32-unknown-unknown` (web).

### The web branch

Browsers expose `localStorage` through JavaScript. We reach it with `document::eval`:

```rust
let eval = document::eval("const key = await dioxus.recv(); return localStorage.getItem('foxg:' + key);");
eval.send(name)?;
eval.join::<Option<String>>().await
```

Note we **send** the name and the text to the script instead of pasting them into the script string. Pasting data into code is how injection bugs happen; sending is just data.

## 6. Your own trait with a generic helper

```rust
pub trait Persist: Sized {
    fn to_text(&self) -> String;
    fn from_text(text: &str) -> Result<Self, ParseError>;
}

pub async fn save_value<T: Persist>(name: &str, value: &T) -> Result<(), StorageError>
```

`Progress` implements `Persist`; `save_value::<Progress>` is generated for it. Tomorrow `Settings` implements it and gets file/web storage **with no new storage code**. That is the point of the abstraction (and the abstraction check passes: why / without / useful).

`Sized` in `Persist: Sized` means "implementors have a known size", needed because `from_text` returns `Self` by value.

`load_value` has **two `?`** on different error types:

```rust
Some(text) => Ok(Some(T::from_text(&text)?)),   // ParseError  -> StorageError (via From)
```
and the earlier `load(name).await?` (already a `StorageError`). `impl From<ParseError> for StorageError` is what makes both compile.

## 7. Parsing text by hand

```rust
let (key, value) = line.split_once('=').ok_or_else(|| ParseError::BadLine(line.to_string()))?;
let number: u32 = value.trim().parse().map_err(|_| ParseError::BadNumber { ... })?;
```

- `split_once('=')` -> `Option<(&str, &str)>`: borrowed slices **into** the line; no allocation.
- `ok_or_else(|| err)` turns `None` into `Err(err)` (the closure runs only when needed).
- `map_err(|_| ...)` converts the error type of a `Result` (here dropping the `ParseIntError`, adding context).
- Both results are then used with `?`.

**Lenient vs strict.** Missing keys use defaults, unknown keys are skipped, blank lines are skipped: a save written by an older or newer version still loads. But a line without `=` or a non-number is an **error**, because guessing what damaged data meant could destroy a good record. Be generous with *differences*, strict with *corruption*.

Because the format is simple text, the tests are simple text too, and you can fix a broken save with Notepad.

## 8. Platform honesty: Android and iOS

`std` has no "app data folder" function. We approximated: `FOXG_DATA_DIR`, `APPDATA`, `HOME/.local/share`, temp dir. On desktop that is right. On Android none of the first three exists for a normal app, so data goes to the **temp dir**, which the system may clear: progress could occasionally reset. Doing this properly needs the platform's app-files path (what the `dirs` crate or Android's JNI provides). That is the cost of the "only std" rule, and the right place to say: *here a small dependency would be justified.* The code is isolated in one function (`data_dir`), so the fix would touch one place.

## 9. Dioxus context: sharing state without passing it down

Progress is needed by Home and four game pages. Passing it as a prop through the router is impossible (pages are created by the router, not by us). **Context** solves this:

```rust
// in App (an ancestor of every page)
let store = use_context_provider(|| ProgressStore { data, notice });

// in any descendant component
let progress = use_context::<ProgressStore>();   // wrapped as use_progress()
```

- The provider stores one value in the component tree; descendants look it up by **type**.
- It is a controlled "global": only components below the provider can see it, and the type is the key.
- `use_context` panics if no provider exists above: a programming error you meet at the first run, not in production.
- We wrap it as `use_progress()` so pages never spell out the type.

### `ProgressStore` is `Copy`

```rust
#[derive(Clone, Copy)]
pub struct ProgressStore { data: Signal<Progress>, notice: Signal<Option<String>> }
```

Both fields are `Copy` handles, so the struct can be `Copy`. Hence `update(mut self, ...)` takes `self` **by value** and still leaves the caller's handle usable: closures capture it by copy.

`update` ends the write guard inside braces **before** `save()` reads the signal: the guard rule from lessons 05 and 11 again. `save` copies the value out (`Progress` is `Copy`), then spawns a task that awaits the write with no guard held.

## 10. Races worth knowing (and why we accept them)

- If the player finishes a game **before** the initial load completes, the load result overwrites the in-memory change (and the next save writes the loaded data). The load takes milliseconds and a game takes seconds; fixing it would need a "loaded?" flag. Written down, not hidden.
- Two browser tabs would overwrite each other's progress (last save wins). A real app would merge or use `storage` events.

## 11. Experiments

1. `cargo run --example exp_file_io`; then change `dir` to a path you cannot write to and read the error.
2. Find `progress.txt` (`%APPDATA%\foxg-hub` on Windows): run `dx serve --platform desktop`, win a game, open the file. Edit it in Notepad and restart.
3. Set `FOXG_DATA_DIR=C:\temp\foxg-test` before starting the desktop app. Where is the file now?
4. Write `snake_best=oops` into the file and start the app: where does the error show up?
5. Add a field `rps_best_streak` to `Progress`. Which tests must change? Does an **old** file still load?
6. Remove `fs::create_dir_all` from `save_to` and delete the folder. What error do you get on the first save? (Run `cargo test`.)
7. Implement `Persist` for a `Settings { theme_dark: bool }` in a test and save/load it with `save_value`.

## 12. Key vocabulary

persistence, file format, `std::fs`, `PathBuf` / `Path`, `io::Error`, `ErrorKind`, `?`, `From`, atomic rename, `cfg`, conditional compilation, trait with generic helper, lenient/strict parsing, context, provider, `Copy` handle, last-write-wins.

## 13. Self-check

- Why does `load` return `Result<Option<String>, _>` and not `String`?
- What does `#[cfg(target_arch = "wasm32")]` do to the code under it on a desktop build?
- Why write to a temp file and rename?
- Why is an unknown key ignored but a non-numeric value an error?
- Which line of `ProgressStore::update` ensures a guard is not alive when saving?
- Why can `ProgressStore` be `Copy` even though it "contains" data?
