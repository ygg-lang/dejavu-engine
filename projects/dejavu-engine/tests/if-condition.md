{% target crates::IO %}

## If condition

{% if a %}
{%- null -%}
{% else %}
{%= true =%}
{% end-if %}
