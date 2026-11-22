mod dict
mod go
mod tauri

mod app 'app/justfile'
mod cli 'cli/justfile'
mod lib 'hanconv/justfile'

mod py 'py/justfile'
mod wasm 'wasm/justfile'

set shell := ["nu", "-c"]
set script-interpreter := ["nu"]
set indentation := "  "
set default-list := true

build: cli::build app::build

dev: app::dev

update:
  cargo update

# Links relative to the repository root only work on GitHub: crates.io resolves them against each
# package directory (`hanconv/`, `cli/`), and PyPI resolves them against
# `pypi.org/project/hanconv/` or `pypi.org/project/hanconv-cli/`. The sync
# therefore rewrites them to absolute GitHub URLs.
[doc("Sync the root English README to the package READMEs")]
[script]
sync-readme:
  let repo = 'https://github.com/fhluo/hanconv'
  let base = $repo + '/blob/main'

  let links = [README.md README.zh-Hans.md]
  let dirs = [hanconv cli py py/cli wasm]

  mut readme = open --raw README.md | decode utf-8
  for link in $links {
    let from = '](' + $link + ')'
    let to = '](' + $base + '/' + $link + ')'
    $readme = $readme | str replace --all $from $to
  }

  $dirs
  | each { |dir| $dir | path join README.md }
  | where { |target| (
    not ($target | path exists)
    or (open --raw $target | decode utf-8) != $readme
  ) }
  | each { |target|
    $readme | save --force $target
    print $"updated ($target)"
  }
  | ignore

[doc("Sync the root LICENSE to the package and module directories")]
[script]
sync-license:
  let license = open --raw LICENSE | decode utf-8
  let dirs = [hanconv cli py py/cli go wasm]

  $dirs
  | each { |dir| $dir | path join LICENSE }
  | where { |target| (
    not ($target | path exists)
    or (open --raw $target | decode utf-8) != $license
  ) }
  | each { |target|
    $license | save --force $target
    print $"updated ($target)"
  }
  | ignore
