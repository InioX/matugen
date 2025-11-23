param(
    [Parameter(Mandatory = $true)]
    [string]$ImagePath
)

cargo run --release --features=dump-json -- image "$ImagePath" `
    --type "scheme-vibrant" `
    --import-json "./example/custom.json" `
    --import-json "./example/custom2.json" `
    --import-json-string "{ \`"text3\`": \`"Hello from args!\`" }" `
    --verbose `
    --config "./example/config.toml" `
    --continue-on-error `
    --fallback-color "#0000ff" `
    --show-colors