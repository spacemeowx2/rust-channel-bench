# Rust channel bench

## tokio runtime

```
running 6 tests
test async_bounded     ... bench:   8,733,684 ns/iter (+/- 991,282)
test async_unbounded   ... bench:   5,325,516 ns/iter (+/- 765,889)
test futures_bounded   ... bench:   8,357,258 ns/iter (+/- 501,241)
test futures_unbounded ... bench:   6,072,374 ns/iter (+/- 253,638)
test tokio_bounded     ... bench:  15,495,656 ns/iter (+/- 981,655)
test tokio_unbounded   ... bench:  10,564,659 ns/iter (+/- 1,396,200)
```

## async-std runtime

```
running 6 tests
test async_bounded     ... bench:  13,899,945 ns/iter (+/- 2,454,511)
test async_unbounded   ... bench:   5,298,671 ns/iter (+/- 804,566)
test futures_bounded   ... bench:  12,650,513 ns/iter (+/- 1,835,403)
test futures_unbounded ... bench:   6,245,074 ns/iter (+/- 873,436)
test tokio_bounded     ... bench:  19,317,162 ns/iter (+/- 2,318,100)
test tokio_unbounded   ... bench:   9,044,005 ns/iter (+/- 320,312)
```
