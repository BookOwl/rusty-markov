# rusty-markov
A random text generator using Markov chains.

Implemented in Rust for <a href="https://bookowl.github.io/2016/12/07/Rusty-(Markov)-Chains/">this blog post</a>.

## Building
1. Install Rust and Cargo
2. Clone this repo and cd into it
3. Run `cargo build --release`
4. Copy target/release/markov to somewhere on your path

## Usage
```
    markov [OPTIONS] <CORPUS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --sentences <SENTENCES>    Sets how many sentences to generate
    -w, --words <WORDS>            Sets how many words to generate

ARGS:
    <CORPUS>    Sets the text corpus to use
```

## License
See LICENSE.txt
