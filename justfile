mod dict
mod go
mod tauri

mod app 'crates/hanconv-app/justfile'
mod cli 'crates/hanconv-cli/justfile'
mod lib 'crates/hanconv/justfile'

mod py 'py/justfile'
mod wasm 'bindings/wasm/justfile'

set shell := ["nu", "-c"]
set script-interpreter := ["nu"]
set indentation := "  "
set default-list := true

build: cli::build app::build

dev: app::dev

update:
  cargo update

# Links relative to the repository root only work on GitHub: crates.io resolves them against each
# package directory (`crates/hanconv/`, `crates/hanconv-cli/`), and PyPI resolves them against
# `pypi.org/project/hanconv/`. The sync therefore rewrites them to absolute GitHub URLs.
[doc("Sync the root English README to the package READMEs")]
[script]
sync-readme:
  let repo = 'https://github.com/fhluo/hanconv'
  let base = $repo + '/blob/main'

  let links = [ 'README.md', 'README.zh-Hans.md' ]
  let targets = [ 'crates/hanconv/README.md', 'crates/hanconv-cli/README.md', 'py/README.md' ]

  mut readme = open --raw README.md
  for link in $links {
    let from = '](' + $link + ')'
    let to = '](' + $base + '/' + $link + ')'
    $readme = $readme | str replace --all $from $to
  }

  for target in $targets {
    $readme | save --force $target
    print $"updated ($target)"
  }
