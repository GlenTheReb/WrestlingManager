# ADR 0001: Tauri, React and Rust

Status: accepted for the foundation.

Use Tauri 2, React 19/strict TypeScript/Vite 8 and Rust 2024. The game needs offline
Windows distribution and responsive, information-dense management screens. Rust owns
canonical state; typed adapters isolate React from native storage.

An Electron shell would offer a consistent bundled browser but increase the runtime
footprint. A fully native UI would avoid web tooling but reduce access to mature table
and accessibility patterns. Tauri uses WebView2 on Windows and therefore needs platform
testing. This is a deliberate engineering trade-off, not a measured memory claim.
