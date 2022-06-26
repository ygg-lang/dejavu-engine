## For loop

### Simple For Loop

```saha
{% for item in items %}
	{%- item -%}
{% end-for %}
```


### For Loop If Empty

```saha
{% for item in items %}
	{%- item -%}
{% else %}
	{%= false =%}
{% end-for %}
```
