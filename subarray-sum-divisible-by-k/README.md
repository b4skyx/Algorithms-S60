# Subarray Sum divisible by K

Problem can be found at [leetcode.](https://leetcode.com/problems/subarray-sums-divisible-by-k/)

Given an array A of integers, return the number of (contiguous, non-empty) subarrays that have a sum divisible by K.

```
Example 1:

Input: A = [4,5,0,-2,-3,1], K = 5
Output: 7
Explanation: There are 7 subarrays with a sum divisible by K = 5:
[4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]

Note:

1 <= A.length <= 30000
-10000 <= A[i] <= 10000
2 <= K <= 10000
```

Approach: To obtain linear time complexity, one can find the cumulative sum reminder array and match for the same occurences which give us the contiguous subset in which the solution is present in.
