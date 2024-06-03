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

# WhiteSpace and NewLine Control

| Symbol | Control                                          |
|:-------|:-------------------------------------------------|
| `{%!`  | Represents text `{%`, does not start a statement |
| `{%`   | `{%.` if expression, `{%-` if statement          |
| `{%.`  | Destroy nothing                                  |
| `{%_`  | Destroy `WS`, except `NL`                        |
| `{%-`  | Destroy `WS` and recent `NL`                     |
| `{%~`  | Destroy `WS` and `NL` except farthest `NL`       |
| `{%=`  | Destroy `WS` and `NL`, all                       |



