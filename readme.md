# Root to digits of pi validator

A validator for entries to a [coding challenge on Code Golf Codidact](https://codegolf.codidact.com/posts/295755).

## :link: Webpage

[Access the validator directly](https://trichoplax.github.io/root-to-digits-of-pi-validator/)

## :wrench: Development

### View
To access the webpage locally for development, you will need an HTTP server because the `.wasm` file will be blocked from loading if the HTML file is opened directly. For example, if you have Python installed, you can enter the following from the `web` directory:

```text
python -m http.server
```

You can then view the page in your browser at:

http://127.0.0.1:8000

### Compile
Changes to the [Rust](https://rust-lang.org/) code require 2 compilation steps due to targeting the web. If you do not already have `wasm-bindgen-cli` installed you can do so with:

```bash
cargo install wasm-bindgen-cli
```

#### Step 1
Compile with Cargo to produce the WASM file in the `target` directory:

```bash
cargo build --target=wasm32-unknown-unknown --release
```

If you do not already have the `wasm32-unknown-unknown` target you can install it with:

```bash
rustup target add wasm32-unknown-unknown
```

If you did not use rustup to install Rust, you may need to use a different approach.

#### Step 2
Use `wasm-bindgen` to optimise that WASM file and include it along with a JavaScript file to make use of it, in the `web` directory:

```bash
wasm-bindgen ./target/wasm32-unknown-unknown/release/root_to_digits_of_pi_validator.wasm --out-dir web --no-typescript --target web
```

GitHub pages is currently configured to serve the contents of the `web` directory. It does no compilation. Pushing changes to the Rust code will have no effect on GitHub pages unless you first compile (both the Cargo and `wasm-bindgen` steps) so that the `web` directory reflects those changes.
