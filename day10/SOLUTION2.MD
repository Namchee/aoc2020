# Solution

At first, I thought that we can solve this using simple backtracking. On smaller inputs, backtracking is fast enough. However, once we try to solve the actual input, your program is too slow to solve the problem (mine never completes).

After some manual simulation, this problem actually tries to solve its subproblems. Realizing that, we can use dynamic programming to solve this problem by memoizing the subproblems result.
The termination condition is when it reaches the last adapter. As our phone is guaranteed to have `3` more jolts than the biggest adapter, we must pick the last adapter, therefore the solution for the last subproblem is `1`.

## Notes

1. **DO NOT FORGET** to push `0` to the array! Without the first zero, the program will generate incorrect results.
2. Is it possible to write the memo in bottom-up manner?
3. Currently, the solution is written in C++11, as the solution contains lots of reference moves.

### Time Complexity: O(n log n)
### Space Complexity: O(n)

