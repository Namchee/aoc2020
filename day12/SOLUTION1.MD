# Solution

[Problem Statement](https://adventofcode.com/2020/day/12#part1)

Just simulate it. To make it easier, store the waypoint increment values in an array. For `L`, we can count it as `R` by substracting 360 with the actual degree.

### Time Complexity: O(n) -> we need to process every input
### Space Complexity: O(1)