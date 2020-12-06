# Solution

A bit tricky, but can be solved by utilizing two pointers. Sort the array, for each element, set the first pointer to the next element and last value of the array. If the sum is higher, move the last pointer backward, else move the next pointer forward.

### Time Complexity: O(n^2)
### Space Complexity: O(1)