param(
    [Parameter(Mandatory = $true)]
    [string]$ImagePath
)

cargo run --release --features=dump-json -- image "$ImagePath" `
    --type "scheme-vibrant" `
    --import-json "./example/custom.json" `
    --import-json "./example/custom2.json" `
    --import-json-string "{ \`"text3\`": \`"Hello from args!\`" }" `
    --debug `
    --config "./example/config.toml" `
    --continue-on-error `
    --fallback-color "#0000ff" `
    --show-colors `
    --alternative-json-output `
    --base16-backend "wal" `
    --lightness-dark -0.1 `
    --lightness-light -0.1 `
    --source-color-index 0 
    # --json hex