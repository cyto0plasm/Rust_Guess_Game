# GuessGame

A simple command-line number guessing game written in Rust.

## Run

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo run
```

## Gameplay

The game generates a random number from **1 to 100**.

* Enter your guess
* Get a hint if your guess is too small or too big
* Win when you guess the correct number
* Invalid input is rejected without ending the game
* Start a new game with `y/n`

Example:

```text
Guess a number
50
You Guessed 50
Too Small!

New Game y/n
y
```

## Built With

* Rust
* Cargo
* `rand`
