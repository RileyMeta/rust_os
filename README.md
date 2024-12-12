# rust_os
A simple x86 Operating System written in Rust
> [!note]
> based on [Philipp Oppermann's blog](https://os.phil-opp.com)

## Necessary Files (see desc.)
```
.
├── src/
│   └── main.rs             # The main program, duh :P
├── .cargo/
│   └── config.toml         # Has the run flags for Cargo
├── cargo.toml              # Dependencies and package info
├── rust-toolchain.toml     # Tell the compiler to use the Rust Nightly without needing to pass `+nightly`
└── x86_64_rust.json        # Target Instructions
```
> The above diagram is for anyone that wants to follow Phillip Oppermann's Blog and make their own Kernel in Rust.
> Specifically `rust-toolchain.toml`, is not mentioned in ^ the above blog, but is necessary on Arch Linux (what I use).
