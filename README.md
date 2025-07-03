# Hypatia-CA

Hypatia-CA is a minimal certificate authority tool written in Rust. It targets
an "offline first" workflow—keys and certificates are generated on an
air‑gapped machine and transferred via removable media. Structured `tracing`
logs and an append-only audit log help track all operations.

The tool currently supports three subcommands:

- `init-root` – generate a self‑signed root certificate
- `sign` – create a detached signature using Falcon or Dilithium
- `revoke` – append a certificate serial number to the revocation list

## Features

- **Pure Rust implementation** using recent crates and no external C
  dependencies.
- **Custom error handling** via an `Error` enum (`src/error.rs`) and a `Result`
  alias. Errors are colored for readability.
- **Tracing based logging** with `tracing-subscriber` (features: `fmt`,
  `env-filter`, `ansi`). Log output can be plain text or JSON.
- **Zeroization of secrets** with the `zeroize` crate.
- **Optional post‑quantum key generation** via `crypt_guard` (Kyber).
- **Digital signatures** via `crypt_guard` (Falcon and Dilithium).
- **Object oriented design** using the `Runnable` trait to execute subcommands.
- **Append-only audit log** written to `/opt/hypatia-ca/audit.log`.

## Directory Layout

```
hypatia-ca/
├── Cargo.toml            # crate manifest
├── src/
│   ├── main.rs           # CLI entry point
│   ├── cmd/              # subcommand modules
│   │   ├── init_root.rs
│   │   ├── sign.rs
│   │   └── revoke.rs
│   ├── util/             # helpers (fs, audit)
│   └── error.rs          # custom error type
└── README.md
```

Certificates are stored under `/opt/hypatia-ca/data/root`. The audit log lives
at `/opt/hypatia-ca/audit.log`.

## Usage

Build the project with a recent Rust toolchain (edition 2024):

```bash
$ cargo build --release
```

Generate a root certificate:

```bash
$ sudo ./target/release/hypatia-ca init-root --cn "Hypatia Root"
```

Use `RUST_LOG=info` or a custom filter to control log output. Passing `--json`
outputs audit events in JSONL format.

## Design Notes

The command parser lives in `src/main.rs` and delegates to modules in
`src/cmd`. Each command implements the `Runnable` trait:

```rust
pub trait Runnable {
    fn run(&self, cli: &crate::Cli) -> Result<()>;
}
```

Errors bubble up as `Result<()>` and are mapped using `map_err`. No calls to
`unwrap()` or `clone()` are used; values are moved or borrowed as needed. Secret
material implements `Zeroize` and is cleared from memory when dropped.

## Security Considerations

- Generated private keys are zeroized after being written to disk.
- Logging includes `debug`, `info`, `error`, and `trace` levels. Events are
  emitted with `event!` macros.
- The audit log is append‑only. Each entry records the timestamp, action and
  details.
- Post‑quantum key generation uses Kyber via `crypt_guard 1.3.10` and supports
  Falcon and Dilithium signatures.

## Contributing

Future work includes adding intermediate CA support and extending the audit log
with integrity checks. Patches and issue reports are welcome.

