# Rust Shellcode Loader

A simple shellcode loader written in Rust for **Red Team Operations**.

The basic idea behind this project is pretty straightforward: the program downloads a `shellcode.bin` file from a remote server and saves it to a specific path defined inside `src/main.rs`.

I built this mainly to experiment with Rust, payload handling, and the kind of file-transfer workflow you might encounter while studying malware and offensive security.

> **Note:** Only use this project on systems you own or have explicit permission to test.

## How It Works

The loader follows a simple workflow:

1. Connects to the configured remote server.
2. Downloads `shellcode.bin`.
3. Stores the downloaded binary at the path configured in `src/main.rs`.

The URL and destination path can be changed directly in the source code.

## Project Structure
```
|--- Cargo.toml
|---- src/
      ^----- main.rs
```
The main logic is inside:

```text
src/main.rs
```

## Requirements

You need the Rust toolchain installed.

Check whether Rust and Cargo are available:

```bash
rustc --version
cargo --version
```

If both commands return a version number, you're good to go.

## Clone the Repository

```bash
git clone <YOUR-REPOSITORY-URL>
cd <REPOSITORY-DIRECTORY>
```

## Configure the Loader

Open:

```text
src/main.rs
```

Inside the file, configure the remote server URL of `shellcode.bin` and the destination path according to your test environment.

Make sure the remote server actually contains the expected file before running the program. After editing the 'main.rs' file go backward one step from the directory and run the below commands where the 'Cargo.toml' file exists.

## Build With Cargo

For a release build of x86_64-pc-windows-gnu:

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The optimized binary will be available in:

```text
target/x86_64-pc-windows-gnu/release/
```

## Run the Project

You can run it directly through Cargo:

```bash
cargo run
```

Or use the release configuration:

```bash
cargo run --release
```

## Useful Cargo Commands

Check the project without producing a final executable:

```bash
cargo check
```

Format the Rust source:

```bash
cargo fmt
```

Clean previously generated build files:

```bash
cargo clean
```

Build a fresh release version:

```bash
cargo clean
cargo build --release --target x86_64-pc-windows-gnu
```

## Testing Environment

I'd recommend testing this inside a VM or another isolated environment rather than directly on your main system.

For example:

```text
┌──────────────────────┐
│   Test HTTP Server   │
│     shellcode.bin    │
└──────────┬───────────┘
           │
           │ HTTP
           ▼
┌──────────────────────┐
│    Rust Loader VM    │
│      main.rs         │
└──────────────────────┘
```

This makes it much easier to experiment without accidentally affecting your normal environment.

## Disclaimer

This project is intended for **educational purposes, Red Team Operations Knowledge and authorized testing**.
Do nt use it to deliver or deploy payloads against systems without permission. You are responsible for making sure your use of this project complies with applicable laws, organizational policies, and the scope of your authorization.
