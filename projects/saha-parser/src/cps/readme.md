## CPS 变换

- 变换前

```scala
loop {
    if (i > 0) {
        i -= 1
        continue
    }
    else {
        break
    }
}
i + 1
```

- 变换后

```scala
def loop_k() {
    if (i > 0) {
        i -= 1
        loop_k()
    }
    else {
        break_k()
    }
}
def break_k() {
    i + 1
}
loop_k()
```