Rustc target triple
===================

Access the $TARGET and $HOST rustc target triples that Cargo supplies to build
scripts.

## Example

```rust
fn main() {
    // As a const &str:
    assert_eq!(target_triple::TARGET, "x86_64-unknown-linux-gnu");

    // As a string literal:
    assert_eq!(
        concat!("target/", target_triple::target!()),
        "target/x86_64-unknown-linux-gnu",
    );
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
