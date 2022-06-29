## If condition



{% if i == 1 %}
{% i %} quick brown fox jumps over the lazy dog!
{% else %}
{% i %} quick brown foxes jump over the lazy dog!
{% end %}



{% if count == 0 %}
no quick brown fox jumps over the lazy dog!
{% else-if count == 1 %}
a quick brown foxes jump over the lazy dog!
{% else %}
many quick brown foxes jump over the lazy dog!
{% end %}