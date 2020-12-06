# Solution

Pretty much straight-forward.

The easiest and fastest way to do this is to utilize the `HashSet` data structure to store all the inputs and facilitate number search.

For each new inputs, try to find `2020 - x` in the `HashSet`. If it's not available, add the new input to the set. If it's available, you'll get the answer :)

### Time Complexity: O(n)
### Space Complexity: O(n)