# Syntax

Brainease has a syntax based on batch files, as you can see, they all have only one instruction per line. Indentation is also pretty simple, just **two** spaces.

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
# Every indentation is made by 2 spaces, no more, no less.
if 2 >= 120

  dec 120 in 2
  # inner comment

  # inner multi-line
  # comment
  if 2 >= 150
    # Inner inner codeblock

    dec 130 in 2


# Instruction out of the if_cell codeblock above
print 2
```
