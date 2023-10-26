### Minigrep Rust project

Can be run with the following command in debug mode:

```
cargo run -- <pattern> <filepath>
```

Or in release mode:

```
cargo run --release -- <pattern> <filepath>
```

Otherwise, build the binary using the following command

```
cargo build --release
```

and then execute it with:

```
./target/release/minigrep -- <pattern> <filepath>
```

## Environment Variables

`IGNORE_CASE`: (Optional) If you want to look for a case insensitive pattern
