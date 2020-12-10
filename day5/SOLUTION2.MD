# Solution

Store the occupied IDs in a `Set`. Then generate every possible ID and check if that ID doesn't exist in the set and both `id + 1` and `id - 1` exists.

### Time Complexity: O(1) -> The simulation has constant bound
### Space Complexity: O(n) -> We need to store all IDs from the input