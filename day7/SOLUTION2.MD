# Solution

Same as before. The funny part of this problem is we only need to modify the previous solution a little bit to solve this one :).

## Notes

1. Currently, the solution is written in Java. Rust implementation is still broken caused by mutable move.
2. The provided DP solution is using the top-down method. Is it possible to generate a bottom-up version?

### Time Complexity: O(n) -> Although we only need to expand 'shiny gold', but the worse case for the expansion is still the expansion of the whole input.
### Space Complexity: O(n)