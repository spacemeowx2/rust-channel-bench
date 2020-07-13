# Rust channel bench

## tokio runtime

```
running 10 tests
test async_bounded            ... bench:   8,519,679 ns/iter (+/- 139,348)
test async_unbounded          ... bench:   5,341,065 ns/iter (+/- 662,362)
test crossfire_mpmc_bounded   ... bench:  10,264,202 ns/iter (+/- 894,351)
test crossfire_mpmc_unbounded ... bench:   5,511,290 ns/iter (+/- 486,210)
test crossfire_mpsc_bounded   ... bench:  10,069,980 ns/iter (+/- 293,518)
test crossfire_mpsc_unbounded ... bench:   5,500,218 ns/iter (+/- 427,863)
test futures_bounded          ... bench:   8,455,769 ns/iter (+/- 358,922)
test futures_unbounded        ... bench:   6,117,978 ns/iter (+/- 233,785)
test tokio_bounded            ... bench:  15,562,061 ns/iter (+/- 2,094,857)
test tokio_unbounded          ... bench:  10,453,023 ns/iter (+/- 1,913,767)
```

## async-std runtime

```
running 10 tests
test async_bounded            ... bench:  13,578,544 ns/iter (+/- 1,554,964)
test async_unbounded          ... bench:   4,701,081 ns/iter (+/- 531,489)
test crossfire_mpmc_bounded   ... bench:  14,881,215 ns/iter (+/- 2,349,798)
test crossfire_mpmc_unbounded ... bench:   5,752,021 ns/iter (+/- 389,811)
test crossfire_mpsc_bounded   ... bench:  14,378,930 ns/iter (+/- 3,322,537)
test crossfire_mpsc_unbounded ... bench:   5,729,744 ns/iter (+/- 307,343)
test futures_bounded          ... bench:  12,605,913 ns/iter (+/- 1,041,504)
test futures_unbounded        ... bench:   7,073,907 ns/iter (+/- 1,305,048)
test tokio_bounded            ... bench:  19,613,914 ns/iter (+/- 712,085)
test tokio_unbounded          ... bench:   9,008,297 ns/iter (+/- 81,043)
```
