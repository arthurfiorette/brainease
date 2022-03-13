# Assignment Operators

In brainease, you can assign and change the value of a specific cell with **4** main
instructions: `inc`, `dec`, `save` and `read`.

### Incrementing and Decrementing

You can increment or decrement a cell with the `inc` and `dec` instructions. They are a
**1:1** mapping to brainf\*ck with the `+` and `-` operators. The only catch is that you
can increment or decrement a cell multiple times in a row.

The syntax is pretty simple:

```r
# Syntax:
(inc|dec) <value> in *<cell>
```

```r
# Increments the cell 2 by 1
inc 1 in *2

# Decrements the pointer cell by 84
dec 84 in *@
```

You need to be careful, because with these two instructions is pretty easy to overflow a
cell. Read more about [cell overflow](../learning/memory.md#cell-overflow).

### Getting ASCII values

To prevent you from programming with a ASCII table on the second monitor, there's a
"special" instruction that allows you to save the ASCII value of a character in a specific
cell.

This improves the readability of your code when interacting with characters. Because
instead of `inc 97 in *5`, you can just write `save 'a' in *5`.

```r
# Syntax:
save '<char>' at *<cell>
```

```r
# Saves the ASCII value of 'a' in the cell 3
save 'a' at *3
# the same as `inc 97 in *3`, but more readable

# Saves the new line character in the cell 6
save '\n' at *6
# the same as `inc 10 in *6`...
```

### Getting characters from STDIN

With the default `Io Handler` implementation, you can read a character from **STDIN** and
save his ASCII value in a specific cell.

To understand better the IO concept and how it works, learn how to
[handle I/O](./handling-io.md) operations.

```r
# Syntax:
read *<cell>
```

```r
# Reads whatever is written in STDIN and saves it in the cell 7
read *7

# Reads the next character from STDIN to the current pointer
read *@
```
