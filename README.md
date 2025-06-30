# Project Setup

This repository hosts a monorepo for a WebAssembly powered Luzhanqi game.

## Tools Used

- **Rust** with Cargo
- **wasm-pack** for building the WASM package
- **Node.js** with **pnpm** as the package manager
- **sv CLI** for generating the SvelteKit UI
- **Turborepo** for orchestrating builds

## Setup Steps

1. Initialize the SvelteKit frontend:
   ```bash
   pnpm dlx sv@latest create apps/ui --template minimal --types ts --install pnpm --no-add-ons
   ```
2. Install `wasm-pack` to build Rust into WebAssembly:
   ```bash
   cargo install wasm-pack
   ```
3. Create Rust crates under `packages/` for `game`, `engine` and `wasm-bindings`.
4. `packages/wasm-bindings/package.json` provides a build script:
   ```json
   {
     "scripts": { "build": "wasm-pack build --target web" }
   }
   ```
5. The workspace is defined in `Cargo.toml` and `pnpm-workspace.yaml` so running `pnpm build` or `cargo build` compiles the crates and frontend together.

Each crate can also be used independently by other Rust projects.
