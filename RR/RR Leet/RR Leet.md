.[[RR Leet]].
  [[LEET keyword-to-algo-mapping]],
  , [[Algo]], [[Trees]], 

  DELS: [[Algo]], 

#leet




### Max Runtimes


Solution Time Complexity by size
O(1),  O(log n), O(n), O(K log n), O(n log n), O(n^2), O(n^2), O(N!)
-?-
. >10^9  >10^8   <10^6  <10^6  <10^6    <3000    <20    <12 <!--SR:!2025-01-23,2,210-->



O(1) tasks
-?-
- Hashmap lookup
- Array access and update
- Pushing and popping elements from a stack
- Finding and applying math formula
- Typically for `n > 10⁹`
<!--SR:!2025-01-28,20,250-->

O(log n) tasks
-?-
- [Binary search](https://algo.monster/problems/binary_search_intro) or variant
- Balanced binary search tree lookup
- Processing the digits of a number
- Typically for `n > 10⁸`
<!--SR:!2025-01-21,13,250-->

O(n) tasks
-?-
- Going through array/linked list
- [Two pointers](https://algo.monster/problems/two_pointers_intro)
- Some types of greedy
- Tree/graph traversal
- Stack/Queue
- Typically for `n ≤ 10⁶`
<!--SR:!2025-01-21,13,230-->

O(K log n) tasks
-?-
- Heap push/pop `K` times. When you encounter problems that seek the "top K elements", you can often solve them by pushing and popping to a heap `K` times, resulting in an `O(K log(N))` runtime. e.g., [K closest points](https://algo.monster/problems/k_closest_points), [merge K sorted lists](https://algo.monster/problems/merge_k_sorted_lists).
- Binary search K times.
- Typically for `n ≤ 10⁶`
<!--SR:!2025-01-28,20,250-->

O(n log n) tasks
-?-
- Sorting. The default sorting algorithm's expected runtime in all mainstream languages is `N log(N).` For example, java uses [a variant of merge sort](https://docs.oracle.com/javase/7/docs/api/java/util/Arrays.html) for object sorting and [a variant of Quick Sort](https://docs.oracle.com/javase/7/docs/api/java/util/Arrays.html#sort(int%5B%5D)) for primitive type sorting.
- [Divide and conquer](https://algo.monster/problems/divide_and_conquer_intro) with a linear time merge operation. Divide is normally `log(N)`, and if merge is `O(N)` then the overall runtime is `O(N log(N))`. An example problem is [smaller numbers to the right](https://algo.monster/problems/count_of_smaller_numbers_after_self).
- Typically for `n ≤ 10⁶`
<!--SR:!2025-01-10,2,210-->

O(n^2) tasks
-?-
- Nested loops, e.g., visiting each matrix entry
- Many brute force solutions
- Typically for `n ≤ 3000`
<!--SR:!2025-01-28,20,250-->

O(2^n) tasks
-?-
- Combinatorial problems, [backtracking](https://algo.monster/problems/backtracking), e.g. subsets
- Often involves recursion and is harder to analyze time complexity at first sight
- Further detailed code examples can be found in the [backtracking](https://algo.monster/problems/backtracking) section
- Typically for `n ≤ 20`
<!--SR:!2025-01-28,20,250-->

O(N!)
-?-
- Combinatorial problems, backtracking, e.g. [permutations](https://algo.monster/problems/permutations)
- Often involves recursion and is harder to analyze time complexity at first sight
- Detailed code examples can be found in the [backtracking](https://algo.monster/problems/backtracking) section
- Typically for `n ≤ 12` <!--SR:!2025-02-23,33,250-->

