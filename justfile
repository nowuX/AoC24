# Configurar PowerShell como shell por defecto
set shell := ["powershell.exe", "-Command"]

year := "2024"

create day:
    cargo new --lib day{{day}} --verbose
    Add-Content -Path "day{{day}}/Cargo.toml" -Value "anyhow = { workspace = true }"
    Add-Content -Path "aoc/Cargo.toml" -Value 'day{{day}} = { version = "0.1.0", path = "../day{{day}}" }'
    Copy-Item "day_template/src/lib.rs" "day{{day}}/src/lib.rs"
    New-Item -ItemType Directory -Path "day{{day}}/assets" -Force
    Set-Location "day{{day}}/assets"; aoc download --year {{year}} --day {{day}} --debug