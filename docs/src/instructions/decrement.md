# Decrement

```r
inc 1 in 4
```

The decrement function will decrement (no way) the specified cell with the specified value.
Value overflow is handled by wrapping the value, so if you decrements a cell with `5` by `20`, the resulting value will be `240`.

```r
# Basic syntax:
# inc {value} in {cell number}

# [0, 0, 0, 0, 0]
dec 10 in 2

# This instruction increments the value of the cell 2 by 10.
# [0, 0, 245, 0, 0]
# But why it was the third cell instead of the second?
# Because the cell number is zero-based.

# This instruction will print the current value of cell 2.
write 2

# (stdout)
#> 245
```
