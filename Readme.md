Coarse timer for when you don't need a completely accurate clock.

# Motivation:

```shell
test get_time::slack_500ms      ... bench:         1 ns/iter (+/- 0)
test get_time::slack_50ms       ... bench:         1 ns/iter (+/- 0)
test get_time::slack_5ms        ... bench:         1 ns/iter (+/- 0)
test get_time::stdlib           ... bench:        52 ns/iter (+/- 37)
test get_time::through_arc_50ms ... bench:         1 ns/iter (+/- 0)
test now::slack_500ms           ... bench:         2 ns/iter (+/- 1)
test now::slack_50ms            ... bench:         3 ns/iter (+/- 2)
test now::slack_5ms             ... bench:         3 ns/iter (+/- 2)
test now::stdlib                ... bench:       757 ns/iter (+/- 232)
test now::through_arc_50ms      ... bench:         3 ns/iter (+/- 2)
test now_utc::slack_500ms       ... bench:         3 ns/iter (+/- 4)
test now_utc::slack_50ms        ... bench:         3 ns/iter (+/- 2)
test now_utc::slack_5ms         ... bench:         3 ns/iter (+/- 1)
test now_utc::stdlib            ... bench:       214 ns/iter (+/- 48)
test now_utc::through_arc_50ms  ... bench:         3 ns/iter (+/- 1)
```
