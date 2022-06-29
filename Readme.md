Déjà-Vu Template Engine
=======================

A modern template engine lessen your pain.

Compile time template engine with zero-cost abstraction.

## Install by Cargo

```sh
cargo install dejavu-engine -f
dejavu --help
```

## File Pattern

It is recommended to name the file as `*.rs.djv`.

|           Input |      Output |
|----------------:|------------:|
| `<NAME>.rs.djv` | `<NAME>.rs` |
|    `<NAME>.djv` | `<NAME>.rs` |
|       `mod.djv` | `<NOTHING>` |