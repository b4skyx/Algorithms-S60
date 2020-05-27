## Minimize the maximum difference between adjacent elements in an array

Problem from [GeeksForGeeks](https://www.geeksforgeeks.org/minimize-the-maximum-difference-between-adjacent-elements-in-an-array/)

Given a non-decreasing array arr[] and an integer K, the task is to remove K elements from the array such that maximum difference between adjacent element is minimum.

Note: K < N – 2


```
Examples:

Input: arr[] = {3, 7, 8, 10, 14}, K = 2
Output: 2
Explanation:
After removing elements A[0] and A[4],
The maximum difference between adjacent elements is minimum.
After removing elements, the remaining array is [7, 8, 10]

Input: arr[] = [12, 16, 22, 31, 31, 38], K = 3
Output: 6
Explanation:
After removing elements A[3], A[4] and A[5],
The maximum difference between adjacent elements is minimum.
After removing elements, the remaining array is [12, 16, 22]
```
