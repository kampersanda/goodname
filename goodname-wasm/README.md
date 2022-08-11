# goodname-wasm

This workspace provides the source code of the [Web App](https://kampersanda.github.io/goodname/).

## How to launch

You can launch the server on your machine with the following commands.

```
# Installs necessary packages.
$ rustup target add wasm32-unknown-unknown
$ cargo install trunk
$ cargo install wasm-bindgen-cli

# Puts your lexcon of words.
$ cp ../wordlist/words.txt src/

# Builds and launches the server.
$ trunk serve --release
```

For ARM Mac, you may need to install `binaryen` to build `wasm-opt`.

```
$ brew install binaryen
```
