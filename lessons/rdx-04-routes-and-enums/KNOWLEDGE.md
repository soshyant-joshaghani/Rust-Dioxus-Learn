# Lesson 04 - KNOWLEDGE (the book part)

---

## 1. Enums: "one of these"

A **struct** says "this AND this AND this" (a game has a title **and** a blurb). An **enum** says "this OR this OR this" (a page is Home **or** Games **or** About).

```rust
enum Light { Red, Yellow, Green }
```

`Light` is a type with exactly three possible values. A variable of type `Light` is **always** one of them: never none, never two.

### Variants can carry data

```rust
enum Shape {
    Circle(f64),             // tuple-like: one unnamed value
    Rect { w: f64, h: f64 }, // struct-like: named fields
    Dot,                     // nothing
}
```

Each variant is its own shape of data, but they are all the same type `Shape`. This is something most languages cannot express cleanly. Our `Route::NotFound { segments: Vec<String> }` is exactly this: the "not found" page is a different *kind* of route because it carries the unknown path.

### How an enum value lives in memory

The compiler stores a small **tag** (which variant) plus enough space for the *largest* variant's data. That is why enums are cheap: no pointers or objects are needed.

## 2. `match`: taking an enum apart

```rust
fn label(self) -> &'static str {
    match self {
        NavItem::Home  => "Home",
        NavItem::Games => "Games",
        NavItem::About => "About",
    }
}
```

- The compiler requires **every variant** be handled ("exhaustive"). If you add `NavItem::Settings`, every `match` that forgot it becomes a compile error, **and the errors are your to-do list**. Adding a feature = following the compiler. This is why Rust refactors are calm.
- Patterns can **destructure**: `Shape::Rect { w, h } => w * h` pulls the fields out into variables.
- Patterns can combine and filter: `1 | 2 => ...`, `n if n > 9 => ...` (a *guard*), ranges `1..=5`.
- `_` means "anything else". Use it sparingly: it switches off the "you forgot a case" safety net. Prefer listing variants.
- `match` is an expression: it produces a value; all arms must have the same type.

Shorter forms:

```rust
if let Some(n) = found { ... }       // match with ONE interesting arm
let ok = matches!(light, Light::Red); // "is it this variant?" -> bool
let Some(n) = found else { return; };  // let-else: bail out early (used later)
```

## 3. `Option<T>` and `Result<T, E>` are just enums

```rust
enum Option<T>    { Some(T), None }
enum Result<T, E> { Ok(T),   Err(E) }
```

- **Rust has no `null`.** "Maybe there is no value" is spelled `Option<T>` in the type, so the compiler forces you to handle `None`. The famous "billion dollar mistake" cannot happen.
- **Rust has no exceptions.** "This can fail" is spelled `Result<T, E>`, and the caller must look at it.

The `<T>` part is a **generic**: `Option<i32>`, `Option<String>`... one definition, many types (lesson 09).

Helpers you will use constantly:

| Method | Does |
|--------|------|
| `opt.unwrap_or(default)` | the value, or a default |
| `opt.is_some()` / `is_none()` | bool |
| `opt.map(f)` | transform the inside if present |
| `res.is_ok()` / `is_err()` | bool |
| `res.unwrap()` / `expect("msg")` | value or **crash**. Fine in experiments, a smell in real code |
| `res?` | "return the error to my caller" (lesson 12) |

Run `cargo run --example exp_enums`.

## 4. `derive`: asking the compiler to write code

```rust
#[derive(Debug, Clone, PartialEq)]
enum Route { ... }
```

`derive` auto-generates standard trait implementations (traits: lesson 09).

| Derive | You get | Typical need |
|--------|---------|--------------|
| `Debug` | `{:?}` printing | debugging output |
| `Clone` | `.clone()` | explicit duplicates |
| `Copy` | implicit duplication on assignment | only for small, plain data |
| `PartialEq` / `Eq` | `==` and `!=` | comparisons, props |
| `Default` | `Type::default()` | "empty" starting values |
| `Hash` | usable as a map key | later |

`Copy` requires `Clone`. A type can be `Copy` only if all its fields are. `NavItem` is `Copy`, so `item.label()` can take `self` by value without "moving" it away from the array.

## 5. The router, in one picture

`#[derive(Routable)]` reads the attributes on each variant and writes two conversions:

```
"/about"  <--->  Route::About {}
"/a/b/c"  <--->  Route::NotFound { segments: ["a","b","c"] }
```

Everything else follows:

| Piece | Meaning |
|-------|---------|
| `Router::<Route> {}` | reads the current URL, shows the matching page |
| `#[route("/path")]` | which URL a variant answers to |
| `#[layout(AppLayout)] ... #[end_layout]` | wrap these pages in `AppLayout` |
| `Outlet::<Route> {}` | inside the layout: "the page goes here" |
| `Link { to: Route::About {} }` | a link that changes the route **without reloading the page** |
| `use_route::<Route>()` | the current route as a value |
| `"/:..segments"` | catch-all that captures the rest of the path |
| `"/user/:id"` | dynamic segment: `Route::User { id: String }` (we use this for games) |

Because links take a `Route` **value**, a broken link is a compile error, not a 404 in production. That is "make illegal states unrepresentable" in action.

Routing order: the first matching route wins, so the catch-all must be last.

### Layout and the "who may import whom" puzzle

`NavLink` needs `Route`. `Header` lives in `modules/base`, which must not import `routes` (dependency rule from lesson 03). Solution: `Header` accepts `children`, and `AppLayout` (in `routes/`, allowed to know `Route`) passes the links in. **The thing that knows too much is the thing placed higher in the dependency order.** This pattern, "give the low-level part a slot instead of knowledge", is a small version of *dependency inversion*.

## 6. Vec<String> (first look at a collection)

`segments: Vec<String>`: a growable list of owned strings on the heap.

```rust
let mut v = Vec::new();   // or vec![1, 2, 3]
v.push(4);                // needs `mut`
v.len();  v[0];  v.iter();
```

- `v[10]` on a 3-element vec **panics** (crash). `v.get(10)` returns `Option<&T>` instead. Prefer `get` when the index comes from outside.
- `segments.join("/")` borrows `segments` (`&self`) and builds a new `String`. The vec is still yours afterwards.

## 7. Where ownership showed up

| Code | Ownership story |
|------|-----------------|
| `Link { to: target, ... }` | `target: Route` is **moved** into `Link` |
| `if current == target` | comparing needs only references: `==` borrows both sides, nothing moves. It is done **before** `target` is moved |
| `for item in NAV_ITEMS` | `NavItem` is `Copy`, so iterating the const array by value is free |
| `fn label(self)` | takes `self` by value; fine because `NavItem` is `Copy` |

Try swapping the order of the `==` line and the `rsx!` that moves `target`. Read the error "borrow of moved value". (It compiles as is because the comparison happens first; the compiler follows the order of execution.)

## 8. Cargo features on dependencies

`features = ["router"]` turns on optional code inside the `dioxus` crate. Features keep compile times and binary sizes down: you pay only for what you enable. Still **one** dependency, only more of it.

## 9. Experiments

1. `cargo run --example exp_enums`; add a variant to `Light` and read the errors.
2. Add `NavItem::Settings` and follow the compiler until it builds (the challenge).
3. Change `Games {}`'s route to `"/play"`. Notice nothing else needs editing: links use the enum, not the string.
4. Visit `/games/` (trailing slash) and `/GAMES`. What happens? Why?
5. Print the route in `NavLink` with `println!("{current:?}")` (use the browser console on web; the terminal on desktop).

## 10. Key vocabulary

enum, variant, tag, `match`, exhaustive, destructure, guard, `if let`, `Option`, `Result`, `derive`, `Copy`, `PartialEq`, route, layout, outlet, slot (`children`), feature flag.

## 11. Self-check

- Why is `Option<T>` better than `null`?
- You add a variant to an enum. What tells you every place that must change?
- Why does `Header` take `children` instead of importing `Route`?
- When can a type derive `Copy`?
