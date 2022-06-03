# boilerplate-rust

[![rust ci](https://github.com/kannkyo/boilerplate-rust/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/kannkyo/boilerplate-rust/actions/workflows/rust-ci.yml) [![rust-clippy analyze](https://github.com/kannkyo/boilerplate-rust/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/kannkyo/boilerplate-rust/actions/workflows/rust-clippy.yml) [![rust publish](https://github.com/kannkyo/boilerplate-rust/actions/workflows/rust-publish.yml/badge.svg)](https://github.com/kannkyo/boilerplate-rust/actions/workflows/rust-publish.yml)

Template for Rust

## Package Layout

```bash
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

## Reference

- [Introduction - The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Introduction - Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
