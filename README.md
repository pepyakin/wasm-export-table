# wasm-export-table

A tool that adds an export entry for the default table.

It's already possible for LLD to produce a binary with an exported table. However, this option is available only for LLD 7, while Rust comes only with LLD 6.

You can use this tool in the meantime until Rust gains LLD 7 support.
