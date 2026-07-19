# rOSox Run Script

This repository contains a Nushell script `run.nu` that:
1. Builds rOSox using `cargo build` (using the default target, which may be set via `.cargo/config` or rustup)
2. Runs the resulting binary in QEMU system emulation

## Prerequisites
- Rust and Cargo installed
- QEMU system binaries available in PATH (e.g., qemu-system-arm, qemu-system-aarch64, etc.)
- The project must produce a binary named `rosox` (default binary name matches package name)

## Usage
From the project root:
```bash
nu run.nu
```

## How it works
- The script determines the target triple used for the build (from cargo config or host fallback)
- Extracts the architecture (e.g., arm, aarch64, x86_64) from the target triple
- Maps the architecture to the appropriate QEMU system binary
- Constructs the binary path: `target/<target>/debug/rosox`
- Runs QEMU with: `qemu-system-<arch> --kernel <binary_path>`

## Notes
- If you get an "unsupported architecture" error, add support for your architecture in the match statement in `run.nu`
- Ensure your `.cargo/config` sets a default target if you intend to cross-compile (e.g., for ARM)