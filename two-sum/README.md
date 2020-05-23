# Two Sum

The problem can be found on [leetcode](https://leetcode.com/problems/two-sum/).

Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

```
Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
```

Approach:

One could permutate though all the possible pairs and find the pair which satisfies the sum, which would have O(n^2) complexity.
To reduce it in a linear time, we can make use of hashset.

For a number k, we need to see if (N-K) exists in the given array. Element lookup complexity of a hashset is O(1) so it is a more appropriate choice in here.

