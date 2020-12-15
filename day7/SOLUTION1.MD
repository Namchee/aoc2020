# Solution

[Problem Statement](https://adventofcode.com/2020/day/7)

At glance, the hardest part of this problem is to parse the input. Fortunately, the input itself has one unique properties: it follows the following pattern `<source> bags contain <amount> <content> bag(s), etc.`. We can exploit that property by splitting the string by the `contain` word, then split each bag contents by `,` and parse those values manually.

To store the bag information, we can store it in a `Map` that contains an array of `Bag` object. To show the list of all bags that a bag may contain, we can use dynamic programming to speed up the search (I think it wouldn't be possible without DP though, as the process is extremely slow). Store the `expanded` indicator in a `Map`, which serves as a clue if we should perform an expansion on that bag or not.

## Notes

1. Currently, the solution is written in Java. Rust implementation is still broken caused by mutable move.
2. The provided DP solution is using the top-down method. Is it possible to generate a bottom-up version?

### Time Complexity: O(n)
### Space Complexity: O(n)