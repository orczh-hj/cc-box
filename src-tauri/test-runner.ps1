# cargo test runner (Windows MSVC target only, wired in .cargo/config.toml)
#
# Why: the lib test exe links tauri/tao and imports comctl32 v6-only entry points
# (SetWindowSubclass etc.). Test exes have no embedded side-by-side manifest, so the
# Windows loader picks comctl32 v5 and the process dies with STATUS_ENTRYPOINT_NOT_FOUND
# at load time. Bin targets embed a manifest via tauri_build and are unaffected.
# This script places a side-by-side <exe>.manifest (loaded automatically by the OS
# when the exe has no embedded one) before running the test exe.
param(
    [Parameter(Mandatory = $true, Position = 0)][string]$Exe,
    [Parameter(ValueFromRemainingArguments = $true)][string[]]$ExeArgs
)

if (Test-Path $Exe) {
    $manifest = "$Exe.manifest"
    if (-not (Test-Path $manifest)) {
        Copy-Item (Join-Path $PSScriptRoot "app.manifest") $manifest -ErrorAction SilentlyContinue
    }
}

& $Exe @ExeArgs
exit $LASTEXITCODE
