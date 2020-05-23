# Leetcode 3sum problem

The problem can be found here

https://leetcode.com/problems/3sum/

---

Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.

Note:

The solution set must not contain duplicate triplets.

Example:

```
Given array nums = [-1, 0, 1, 2, -1, -4],

A solution set is:
[
  [-1, 0, 1],
  [-1, -1, 2]
]
```

Approach:

One way to solve this problem is using the brute force technique, which would be non efficient and time consuming.
The brute force approach calculates app possible combinations and checks for validity.

Other way we could solve this problem is by converting it into a 2sum problem which takes linear time to solve.
In this, we fixed a number and checked if the ``sum - num`` is in the array or not. If it is, then solution exists else it does not.
