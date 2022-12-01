
# AOC - 1

- Link to [puzzle](https://adventofcode.com/2022/day/1)

## Input

``` r
library(magrittr)
test_input = c(1000L,
               2000L,
               3000L,
               "",
               4000L,
               "",
               5000L,
               6000L,
               "",
               7000L,
               8000L,
               9000L,
               "",
               10000L)

input_fn = "input.txt"
input = readLines(input_fn)
```

## Function

``` r
p1 <- function(x, n = 1){
  # patch
  x = c(x,"") 
  sep = (x == "") %>% which()
  tots = length(sep) 
  res = double(tots)
  iidx = 1
  for (i in seq_along(res)) {
    eid = sep[i] - 1
    res[i] = sum(as.numeric(x[iidx:eid]))
    iidx = sep[i] + 1
  }
  sum(rev(res[order(res)])[1:n])
}
```

## Execution

``` r
## p1
stopifnot(p1(test_input) == 24000)
p1(input)
```

    ## [1] 66616

``` r
## p2
stopifnot(p1(test_input,n =3) == 45000)
p1(input, n = 3)
```

    ## [1] 199172

## Bench

| expression |   min | median | mem_alloc | n_itr |
|:-----------|------:|-------:|----------:|------:|
| p11        | 237µs |  241µs |    44.4KB |     5 |
| p12        | 221µs |  228µs |    44.4KB |     5 |
