## Value

### Atomic value

|      Input      |  Output  |  Type   |                  Comment                  |
|:---------------:|:--------:|:-------:|:-----------------------------------------:|
|  `{% null %}`   |   ` `    |  Null   |  Remove directly, equipment to `{% _ %}`  |
|  `{% true %}`   |  `true`  | Boolean |                                           |
|  `{% false %}`  | `false`  | Boolean |                                           |
|    `{% 0 %}`    |   `0`    | Number  |                                           |
|  `{% 1**5 %}`   | `10000`  | Number  | scientific notation, 5 significant digits |
| `{% 1.0**-3 %}` | `0.0010` | Number  | scientific notation, 4 significant digits |