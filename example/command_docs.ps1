param(
    [Parameter(Mandatory = $true)]
    [string]$ImagePath
)

$cargoOutput = cargo run --release --all-features -- image "$ImagePath" `
    --type "scheme-vibrant" `
    --import-json "./example/custom.json" `
    --import-json "./example/custom2.json" `
    --import-json-string "{ \`"text3\`": \`"Hello from args!\`" }" `
    --config "./example/config.toml" `
    --continue-on-error `
    --fallback-color "#0000ff" `
    --alternative-json-output `
    --base16-backend "wal" `
    --filter-docs-html `
    --quiet
    # --show-colors `
    # --verbose `
    # --json hex

$cargoOutput | Set-Clipboard