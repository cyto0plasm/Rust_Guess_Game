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
* **Too Small!** if your guess is below the secret number
* **Too Big!** if your guess is above the secret number
* **You Win** when you guess correctly
* Invalid input is rejected
* Start a new game with `y/n`

Example:

```text
Guess a number
1
You Guessed 1
Too Small!
New Game y/n
y
Guess a number
99
You Guessed 99
Too Big!
New Game y/n
```

A correct guess displays **You Win** in green.

## Built With

* Rust
* Cargo
* `rand`
