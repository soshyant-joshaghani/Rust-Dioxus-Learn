# Lesson 09 - KNOWLEDGE (the book part)

Read with `src/modules/games/mod.rs`, `src/modules/base/hooks.rs` and `examples/exp_traits_generics.rs`.

---

## 1. The problem generics and traits solve

You wrote `describe` for Guess, then again for Rps, then again for Tic-Tac-Toe. The bodies differ, the *shape* is identical: "give me the status of a game". Two questions appear:

1. How do I say "these different types share an ability"? -> **trait**.
2. How do I write one function that works for all of them? -> **generics**.

## 2. Traits

```rust
trait Game {
    fn is_over(&self) -> bool;                 // required
    fn summary(&self) -> String;               // required
    fn headline(&self) -> String { ... }       // default (optional to override)
}

impl Game for GuessGame { ... }               // GuessGame keeps the promise
```

A trait is a **named set of methods** a type can implement. It is Rust's version of an *interface*, but far more flexible: you can implement your trait for types you did not write (e.g. `impl Game for i32`), and standard traits for your own types.

You have already used many:

| Trait | What it gives | Where we used it |
|-------|---------------|------------------|
| `Debug`, `Clone`, `Copy`, `PartialEq`, `Default` | derive-able basics | everywhere |
| `Display` | `{}` and `.to_string()` | `GuessError`, `Move` |
| `From<T>` | conversion, powers `?` | `GuessError: From<ParseIntError>` |
| `Iterator` | `map`, `filter`, `for`, `collect`... | `Countdown` in the experiment |
| `Error` | "a proper error type" | `impl Error for GuessError` |
| `Default` | `Type::default()` | `Score`, `Cell` |

### Rules worth knowing

- **Required methods** (no body) must be implemented. **Default methods** (with a body) come free; you may override them.
- A trait's methods can be called **only when the trait is in scope**. `use crate::modules::games::Game;` in the pages is not decoration: remove it and `g.is_over()` fails with "no method found... items from traits can only be used if the trait is in scope".
- **Orphan rule**: you may implement a trait for a type only if *either* the trait *or* the type is yours. So `impl Display for Vec<i32>` is forbidden (neither is yours), but `impl Game for Vec<i32>` is fine (the trait is yours). This keeps two libraries from implementing the same thing differently.
- Traits can require other traits: `trait Pet: Animal + Display`.

## 3. Generics

```rust
fn describe<G: Game>(game: &G) -> String { game.headline() }
```

- `<G: Game>` introduces a **type parameter** `G` with a **bound**: "any type, provided it implements `Game`".
- Inside the function you can use only what the bound promises. `game.headline()` is fine; `game.secret` is not, because `G` might not have it. The compiler checks the generic function **once**, against the bound, not once per use.
- The same code, three equivalent spellings:

```rust
fn describe<G: Game>(game: &G)            // bound inline
fn describe(game: &impl Game)             // `impl Trait` shorthand
fn describe<G>(game: &G) where G: Game    // `where` clause: nicer when bounds are long
```

Multiple bounds: `T: Display + Clone`. Generic *types* also exist: `Stack<T>`, `Option<T>`, `Vec<T>`, `Signal<T>`, `Router::<Route>`: every `<...>` you met since lesson 04 was a generic parameter.

### Monomorphization (why generics are free)

For each concrete type used, the compiler generates a specialised copy:

```
describe::<GuessGame>   describe::<RpsGame>   describe::<Board>
```

Calls are direct, can be inlined, no run-time lookup. Cost: code size. This is **static dispatch**.

## 4. `dyn Trait`: dynamic dispatch

Generics fix the type at compile time. Sometimes you want *different types in one collection*:

```rust
let all: [&dyn Game; 2] = [&guess, &board];
```

- `dyn Game` means "some type implementing Game, **found out at run time**".
- A reference or box to it is a "fat pointer": data pointer + a **vtable** (table of the methods). Each call looks the method up in the vtable.
- You must put it behind a pointer (`&dyn`, `Box<dyn>`) because its size is unknown.

| | `fn f<T: Game>(x: &T)` | `fn f(x: &dyn Game)` |
|---|---|---|
| decided | at compile time | at run time |
| speed | fastest, inlinable | one indirect call |
| binary size | one copy per type | one copy |
| mixed collection | no | yes |
| trait restrictions | none | must be "object safe" (no generic methods, no `Self` returns...) |

Rule of thumb: **start generic; reach for `dyn` when you need a heterogeneous list or want to shrink code size.**

## 5. Function pointers and closures as parameters

```rust
pub fn use_seeded<G: 'static>(make: fn(u64) -> G) -> ...
let (game, restart) = use_seeded(GuessGame::from_seed);
```

- `fn(u64) -> G` is a **function pointer type**. `GuessGame::from_seed` is a normal function, so it fits; no closure needed.
- A closure that captures variables does **not** fit `fn(...)`. For that use a generic parameter `F: Fn(u64) -> G` (what `Iterator::map` does). We chose the simpler `fn` because our constructors capture nothing.
- Returning `impl Fn() + Copy + 'static`: "I return some closure; you may call it many times and copy it". The real type is unnameable (every closure has its own), and `impl Trait` in return position hides it.

## 6. The `'static` bound

`G: 'static` does **not** mean "lives forever". It means "**contains no borrowed references that could expire**". Owned types (`GuessGame`, `String`, `Vec<T>`) qualify. A struct holding `&'a str` for a short `'a` does not. Signals store their value in global storage that outlives any function, so they demand `'static`. You will see this bound whenever data is stored away (signals, threads, `Box<dyn Trait>`).

## 7. Designing the trait: what to put in it

A tempting extra method would be `fn name() -> &'static str`. An associated function **without `self`** makes the trait unusable as `dyn Game` (there is no value to look the method up on). The trait we wrote has only methods with `&self`, so it works as a generic bound **and** as `dyn`. Smaller traits are easier to implement and to combine.

Why `is_over` + `summary` and not `fn won(&self) -> bool`? Because the *three* games would need to answer differently: Rock Paper Scissors never ends, Tic-Tac-Toe has three states. The trait states **only what all of them can honestly answer**.

## 8. The abstraction check, honestly

| Question | `Game` + `describe` | `use_seeded` |
|----------|--------------------|---------------|
| Why does it exist? | pages need "status text" for any game | seed + async + restart was copy-pasted |
| Without it | per-game text code in each page | 8 duplicated lines per game, Snake would be a third copy |
| Why useful? | adding a game = one `impl`, pages unchanged | adding a game = one line in the page |

If you cannot fill this table in, do not abstract. We waited until there were **three** games before introducing either: with only one, we would have guessed the wrong shape.

## 9. Tests with a fake type

```rust
struct Dummy(bool);
impl Game for Dummy { ... }
assert_eq!(Dummy(true).headline(), "Finished. dummy");
```

Because `headline` only needs the *trait*, we test it with a trivial fake instead of a full game. Traits give you seams for testing.

## 10. Ownership and borrowing notes

- `describe(&*board.read())`: `board.read()` is a guard; `*guard` is the `Board` inside; `&` borrows it. Two steps: get access, then borrow the thing. (`&board.read()` would also work through auto-deref in many cases; the explicit form shows what happens.)
- `map_or(String::from("Starting..."), describe)`: `describe` is passed as a **function** to `map_or`; its argument type is `&G`, matching `Option<&G>` from `as_ref()`. Functions are values.
- `use_seeded` returns `(Signal, closure)`: both are `Copy`, so the page uses them in several closures without cloning.

## 11. Experiments

1. `cargo run --example exp_traits_generics`. Predict each line first.
2. Remove `use crate::modules::games::Game;` from `routes/games/guess.rs` and read the message.
3. In the experiment, call `announce(&5)` and `let mixed = vec![Dog, Cat { lives: 1 }];`. Two different errors: bounds vs one-type vectors.
4. Add `fn name(&self) -> String` to `Game` (required) and implement it. Then try `let all: [&dyn Game; 2]`. Still compiles? Now change it to `fn name() -> &'static str` and look at the `dyn` error.
5. Implement `Game` for `Option<Board>`: "no board yet" when `None`. Does `describe(&None::<Board>)` work?
6. Count: how many lines would a fourth game's page need if it did not use `use_seeded`?

## 12. Key vocabulary

trait, required / default method, `impl Trait for Type`, trait bound, generic function / type, type parameter, monomorphization, static dispatch, `dyn Trait`, vtable, trait object, object safety, function pointer, `impl Trait`, `'static` bound, orphan rule, associated type.

## 13. Self-check

- What is the difference between `fn f<T: Game>(t: &T)` and `fn f(t: &dyn Game)`?
- Why does `Vec<Dog>` not accept a `Cat`, and how do you fix it?
- Why must a trait be in scope to call its method?
- What does `G: 'static` really require?
- When should you NOT introduce a trait?
