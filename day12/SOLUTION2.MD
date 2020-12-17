# Solution

The solution has been rewritten from scratch.

We can handle `N`, `S`, `W`, `E`, `F` command easily. The hard part for me is handling the rotation query, since I have forgotten about it. If we want to rotate a point with certain angle, we can use

```
x = x * cos(angle) - y * sin(angle)
y = x * sin(angle) + y * cos(angle)
```

> Derived from [Wikipedia](https://en.wikipedia.org/wiki/Rotations_and_reflections_in_two_dimensions)

The second important point is **DO NOT** convert the floating point directly to integers, call `round()` first. Without rounding the value, the rotation result is **wrong**.

### Time Complexity: O(n)
### Space Complexity: O(1)