# Hypatia-CA

Hypatia-CA is a small certificate authority written entirely in Rust. It is designed around an **offline root** model and an auditable workflow.

## Subcommands

- `init-root` – create a self‑signed root certificate
- `sign-cert` – sign a certificate with the root CA
- `signature` – sign or verify files using Falcon or Dilithium
- `revoke` – add a certificate serial to the revocation list
- `serve` – run a local HTTP API for certificate requests

## Features

- Pure Rust with a custom `Error` type and colored formatting
- Logging via `tracing` with `fmt`, `env-filter` and colored output
- Zeroization of private key material
- Falcon and Dilithium signatures via `crypt_guard` 1.3.10
- X.509 certificate creation using `rcgen`
- Append‑only audit log at `/opt/hypatia-ca/audit.log`

## Security Plan

1. **Sovereignty of Root Trust** – the root CA is generated offline and never used for automatic issuance. Certificates are normally signed by an intermediate CA.
2. **Key Custody & Hardware Backing** – keys should be stored in hardware (HSM or secure enclave). Root keys are ideally cold stored.
3. **Certificate Profiles** – SANs are restricted and lifetimes kept short. Extensions set basic constraints and EKUs.

## Directory Layout

```
hypatia-ca/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cmd/
│   │   ├── init_root.rs
│   │   ├── sign_cert.rs
│   │   ├── signature.rs
│   │   ├── revoke.rs
│   │   └── serve.rs
│   ├── util/
│   │   ├── fs.rs
│   │   └── audit.rs
│   └── error.rs
└── README.md
```

Generated material is written below `/opt/hypatia-ca/data`. Run all commands with `sudo` so the tool can write there.

## Usage

Build with a recent toolchain:

```bash
$ cargo build --release
```

Create the root certificate:

```bash
$ sudo ./target/release/hypatia-ca init-root --cn "Hypatia Root"
```

Sign a certificate:

```bash
$ sudo ./target/release/hypatia-ca sign-cert --cn "example.com"
```

Sign a file:

```bash
$ sudo ./target/release/hypatia-ca signature --file example.txt --sign
```

Run a local API:

```bash
$ sudo ./target/release/hypatia-ca serve --addr 127.0.0.1:8080
```

Development uses `cargo fmt --all`, `cargo clippy`, and `cargo test`.
