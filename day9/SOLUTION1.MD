# Solution

[Problem Statement](https://adventofcode.com/2020/day/9)

For more efficient solution, we need to keep track on the currently valid number - which is 25 numbers before the current number. As we also need to count for the frequency to prevent duplicates, we can't use `Set` for this and must rely on `Map`. To discard the previous number and insert a new number to the cache easily, we can use `Queue` as we can't rely on `Map` to guarantee the insertion order when we loop over it.

For each `26th` number, find a pair of number from the previous 25 which sums to the current number. Since we are using `Map` to solve this, finding the pair number is really fast.

## Notes

1. The solution generates a warning when compiled however, the program still generates correct answer. 

### Time Complexity: O(1) -> Looping the map is always constant
### Space Complexity: O(1) -> The contents of the map and set is always constant