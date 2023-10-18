# storage

Sync key-value storage abstraction over RAM/remote storage engine.

## Assumptions

- We wouldn't want to change the interface of the `RedisMock`.

## Checks

```sh
cargo test && cargo clippy && cargo +nightly fmt
```

## Example

Check out `tests` directory for usage examples.
