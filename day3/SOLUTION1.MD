# Solution

Pretty much straight-forward. The main problem is just input handling. Although you have to watch out for index out of bounds when traversing the columns. To handle that without IFs, we can `y = (y + 3) % <column_size>`;

### Time Complexity: O(x) -> row size
### Space Complexity: O(1)