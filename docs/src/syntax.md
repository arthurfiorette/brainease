# Syntax

## Instructions

The syntax of this language is very simple. For each line, you have one instruction.

```r
instruction_1

instruction_2

# Comments starts with '#'
```

## Learn by example

You can learn the whole language syntax by looking at this example. Focus on the syntax, not on the instructions name, that we will cover later

```r
# This is a comment
# Empty lines are ignored

# this is a
# multi-line comment

# this is a instruction
inc 123 in 2

# Instruction that has inner codeblocks
if 2 >= 120

  dec 120 in 2
  # inner comment

  # inner multi-line
  # comment
  if 2 >= 150
    # Inner inner codeblock

    dec 130 in 2


# Instruction out of the if_cell above
print 2
```
