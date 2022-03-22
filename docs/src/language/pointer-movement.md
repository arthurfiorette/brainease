# Pointer Movement

Sometimes, you'll need the flexibility to read and manipulate data cells without knowing
their exact position. For example, you may need to save N cells depending in the size of
the provided STDIN input.

> The pointer value starts at **`0`**.

This can be managed by the `@` _(A.K.A pointer)_ cell index. Anywhere that a cell can be
referenced, `inc 1 in *1` for example, you can also use the `@` pointer. `inc 1 in *@` in
this example.

You can also move the pointer around with the `goto` operator.

## The `goto` operator

There are 2 sides to move the pointer and 2 ways for each side.

You can see the dedicated [`goto`](../instructions/goto.md) instruction page for more
details.

```r
# Moves to left by one cell
goto left

# Moves the pointer to the left by 5 cells
goto left by 5

# Moves to right by one cell
goto right

# Moves the pointer to the right by 5 cells
goto right by 5
```

## Pointer overflow

If the pointer is current at a edge cell _(i.e. `0` or `29 999`)_, and you move it in the
opposite direction, you'll have the `pointer overflow` effect. It is pretty simple and
easy to understand.

Look at this code example:

```r
# This example is using --memory=10 to allocate only 10 cells

write *@ # Pointer is now at the first cell (0)

# Pointer: *0 --- *(Max Memory Size)
# Pointer: *0 -> *9 (Because 0 to 9 is 10 cells)
goto left

write *@ # Pointer is now at the last cell (9)
```
