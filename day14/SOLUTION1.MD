# Solution

[Problem Statement](https://adventofcode.com/2020/day/14#part1)

The input parsing is quite simple actually, we can use `regex` to make the parsing easier. The main focus of this problem is `how to set the n-th bit with x`.

To set the `n-th` bit with `x` on `a`:
1. Set the `n-th` on `a` to zero, which can be done by `AND`-ing the current value with the negation of `(1 << n)`.
2. Set the `n-th` on `a` with `x`, which can be done by `OR`-ing the value with `(x << n)`.

In short, the above code can be written in one-liner solution as:

```
value = (value & ~(1 << n)) | (x << 1)
```

> I think there's a simpler solution than this, but this is much easier to understand IMO.

### Time Complexity: O(1)
### Space Complexity: O(1)