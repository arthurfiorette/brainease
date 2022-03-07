# Syntax

The syntax of brainease code is similar to _batch_, because each instruction is in his
line of code.

```r
instruction 1
instruction 2
instruction 3
# and so on...
```

# Indentation System

Some instructions may requires his owns inner code, like `loop` or `if`. In this case, you
can indent them with `2` spaces. No more, no less.

```r
instruction 1
  # This instruction belongs to the instruction 1 above.
  # And may, or may not be executed
  instruction 2

    # And this to the instruction 2.
    instruction 3

  # This still belongs to the instruction 1
  instruction 4

# And this one is outside the instruction 1 indentation scope.
instruction 5
```

# Identifying `Pointers`, `Cells`, `Numbers` and `Chars`.

Brainease syntactically identifies `pointers`, `cells` and `values` by the following
rules:

- `*@` is a raw pointer. It starts at `0` and can be incremented or decremented by the
  [`goto`](./instructions.md#goto) instruction.

- `*<number>` represents a memory cell. Ex: `*2` refers to the cell at the index 2.

- `'<char>'` is a character. It supports the `\` escape sequence and internally is
  converted to a `number` by getting his ASCII code. Ex: `'a'` is converted to `97`, if
  needed.

- `<number>` is a raw number. Depending on the instruction, it can be used to increment a
  value, decrement or be a condition.

# Learn by example

There are plenty examples at the
[**examples**](https://github.com/arthurfiorette/brainease/tree/main/examples) folder. But
this simpler one may help you to get started:

```r
# This is a comment
# Empty lines are ignored
# This is also a multi-line comment

# Increments 123 in the current cell
inc 123 in *2

# If the cell 2 is bigger or equal to 120, then print the char 'H' to stdout.
if *2 >= 120
  print 'H'

# This will move the pointer to the right until the pointer
# reaches a cell with a value of 0
loop *@
  goto right


# this is a instruction
inc 123 in *2 # 123 is a raw number and *2 is a memory index
```
