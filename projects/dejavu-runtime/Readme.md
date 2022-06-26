
## Comment

- surrounded by parentheses `{#` and `#}`

```dejavu
{% filter comment %}

comment block

{% end %}
```



## Whitespace Control

**DejaVu Supports `LF` only!**

- `{%=`: Destroy all whitespace on the left
- `{%-`: Destroy all blank lines on the left
- `{%_`: Destroy left whitespace, and the first newline encountered
- `{%!`: destroy left whitespace, but exclude newlines


- `=%}`: Destroy all whitespace on the right
- `-%}`: Destroy all blank lines on the right
- `_%}`: Destroy right whitespace, and the first newline encountered
- `!%}`: destroy right whitespace, but exclude newlines

