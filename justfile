# Configurar PowerShell como shell por defecto
set shell := ["powershell.exe", "-Command"]

year := "2024"

check:
    cargo check
    cargo test

create day:
    Copy-Item src/bin/day_template.rs src/bin/day{{day}}.rs
    aoc download --year {{year}} --day {{day}} --debug -I -i input/{{day}}.in
    just check
