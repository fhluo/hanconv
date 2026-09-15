set shell := ["nu", "-c"]
set script-interpreter := ["nu"]
set indentation := "  "
set default-list := true

[script]
update:
  let dirs = [ crates/hanconv/data go/dict/data ]
  mkdir ...$dirs

  let files = (
    http get https://api.github.com/repos/byvoid/opencc/contents/data/dictionary
    | where name ends-with .txt
    | select name download_url
  )
  let total = $files | length

  for item in ($files | enumerate) {
    let i = $item.index
    let file = $item.item
    let content = http get $file.download_url

    print $"(ansi cyan)[($i + 1)/($total)](ansi reset) Downloading (ansi yellow)($file.name)(ansi reset)"

    for dir in $dirs {
      $content | save -f $"($dir)/($file.name)"
    }
  }
