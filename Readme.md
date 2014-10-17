Coarse timer for when you don't need a completely accurate clock.

# Motivation:

```shell
test slack_5ms   ... bench:         1 ns/iter (+/- 0)
test slack_50ms  ... bench:         1 ns/iter (+/- 1)
test slack_500ms ... bench:         1 ns/iter (+/- 0)
test stdlib      ... bench:        58 ns/iter (+/- 20)
```
