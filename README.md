
# Joke_cli

A CLI program that uses the [JokeAPI](https://jokeapi.dev/) to give you the joke that you need


## Installation

* Install my-project with cargo

```bash
  cargo install joke_cli
```

* Compile it yourself
```bash
git clone https://github.com/quotequack/joke_cli
cd joke_cli
cargo build  --release
```
Binary or exe will be under joke_cli/target/releases
## Usage

```bash
USAGE:
    joker <DELAY> <CATEGORY>

ARGS:
    <DELAY>       Enter a delay between command and print joke
    <CATEGORY>    Select a category: 1. Misc 2. Programming 3. Dark 4. Pun

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

### Delay
The delay between you sending the command and the program typing it for you in any text box you have selected (useful for clicking on the text box in time)
### Category
Select between Programming Dark Pun and Misc (use the category name *not* the number)
## Examples
#### For a dark joke with a reasonable delay
```bash
joker 5 dark
```
#### For a pun with short delay
```bash
joker 2 pun
```


