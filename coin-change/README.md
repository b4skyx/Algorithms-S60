# Coin Exchange Problem

This can be found on [Hackerrank](https://www.hackerrank.com/challenges/coin-change/problem) and on [Leetcode.](https://leetcode.com/problems/coin-change-2/submissions/)

You are working at the cash counter at a fun-fair, and you have different types of coins available to you in infinite quantities. The value of each coin is already given. Can you determine the number of ways of making change for a particular number of units using the given types of coins?

For example, if you have  types of coins, and the value of each type is given as  respectively, you can make change for  units in three ways: , , and .

Function Description

Complete the getWays function in the editor below. It must return an integer denoting the number of ways to make change.

getWays has the following parameter(s):

n: an integer, the amount to make change for
c: an array of integers representing available denominations

Output Format

Print a long integer denoting the number of ways we can get a sum of  from the given infinite supply of  types of coins.

```
Sample Input 0

4 3
1 2 3
Sample Output 0

4
```
