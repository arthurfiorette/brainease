# Memory

Brainease has a memory system identical to the brainf\*ck language. It simply a two
dimensional array of `8 bit` integers.

The memory array can have any length, but the default is 30 000, identical to the BF
standard. You can change the size of the array before running the program with the `-m`
flag.

This is the exemplified memory for a program run with `--memory 5`

<bf-memory length="5"></bf-memory>

And you can manipulate with some of the following commands:

```r
# Increments cell 2 by 187
inc 187 in *2
```

<bf-memory length="5" c2="187"></bf-memory>

```r
# Moves the pointer to left by 4 cells
goto left by 4
```

<bf-memory length="5" pointer="4"></bf-memory>

## Pointers

An important and basic concept inherited from the brainf\*ck language is the pointer.

A pointer is a special value that can point to a cell in the memory. But, when you change
the pointer's value, it will only change to what cell is being pointed to.

[Read more about pointer movement](../language/pointer-movement.md).

<bf-memory length="5"></bf-memory>

```r
# Moves the pointer 3 cells to the right
goto right by 3
```

<bf-memory length="5" pointer="3"></bf-memory>

## Cell Overflow

Each cell has the range of `0-255`, if you try to increment a cell that overflows the
maximum value, the resulting sum will wrapped and the resulting value will be modulo of
`256`.

<bf-memory length="5" c3="254"></bf-memory>

```r
# Increments the 3rd cell by 2
inc 3 in *3
```

<bf-memory length="5" c3="1"></bf-memory>

**Explanation**: `254 (+1) -> 255 (+1) -> 0 (+1) -> 1`

<br />

The same occurs when you try to decrement a cell:

<bf-memory length="5" c3="2"></bf-memory>

```r
# Decrements the 5th cell by 5
dec 5 in *3
```

<bf-memory length="5" c3="253"></bf-memory>

**Explanation**: `2 (-1) -> 1 (-1) -> 0 (-1) -> 255 (-1) -> 254 (-1) -> 253`
