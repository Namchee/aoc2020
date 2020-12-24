# Solution

[Problem Statement](https://adventofcode.com/2020/day/15)

At first, I thought that this problem must be solved by [cycle-finding algorithms](https://en.wikipedia.org/wiki/Cycle_detection). However, it is highly possible that the input won't contain any cycles at all. The solution itself is quite simple, simulate it efficiently.

To simulate efficiently, don't search the previous number by looping, instead store the previous indices with a `HashMap`.

### Time Complexity: O(1)
### Space Complexity: O(1)