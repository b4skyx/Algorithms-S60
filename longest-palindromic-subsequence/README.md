# Longest Palindromic Subsequence

Problem can be found at [Leetcode.](https://leetcode.com/problems/longest-palindromic-subsequence/)

Given a string s, find the longest palindromic subsequence's length in s. You may assume that the maximum length of s is 1000.

```
Example 1:
Input:

"bbbab"
Output:
4
One possible longest palindromic subsequence is "bbbb".
Example 2:
Input:

"cbbd"
Output:
2
One possible longest palindromic subsequence is "bb".
```

Approach: Using dp, one can dynamically move front and end position, making subsequences.
