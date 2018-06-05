# wasm-export-table

Tool that adds an export of the default table.

It's already possible to make LLD produce a binary with an exported table. 
However, this option is available only for LLD 7, while Rust comes only with LLD 6.

Until Rust gains LLD 7 support you can use this tool in the meantime.
