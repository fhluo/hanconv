mod dict
mod go
mod tauri

mod app 'crates/hanconv-app/justfile'
mod cli 'crates/hanconv-cli/justfile'
mod lib 'crates/hanconv/justfile'

mod wasm 'bindings/wasm/justfile'

set shell := ["nu", "-c"]
set script-interpreter := ["nu"]
set indentation := "  "
set default-list := true

build: cli::build app::build

dev: app::dev

update:
  cargo update
