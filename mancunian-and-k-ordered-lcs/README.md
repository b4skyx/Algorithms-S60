# Mancunian and K-Ordered LCS

The problem can be found on [hackerearth.](https://www.hackerearth.com/problem/algorithm/mancunian-and-k-ordered-lcs-e6a4b8c6/description/)

Any programmer worth his salt would be familiar with the famous Longest Common Subsequence problem. Mancunian was asked to solve the same by an incompetent programmer. As expected, he solved it in a flash. To complicate matters, a twist was introduced in the problem.

In addition to the two sequences, an additional parameter k was introduced. A k-ordered LCS is defined to be the LCS of two sequences if you are allowed to change at most k elements in the first sequence to any value you wish to. Can you help Mancunian solve this version of the classical problem?

Input:
The first line contains three integers N, M and k, denoting the lengths of the first and second sequences and the value of the provided parameter respectively.
The second line contains N integers denoting the elements of the first sequence.
The third line contains M integers denoting the elements of the second sequence.

Output:
Print the answer in a new line.

```
Constraints:
1 <= N, M <= 2000
1 <= k <= 5
1 <= element in any sequence <= 109
```

```
SAMPLE INPUT
5 5 1
1 2 3 4 5
5 3 1 4 2
SAMPLE OUTPUT
3
```
Explanation/Approach
You can change the first element of the first sequence to 5 to get the LCS comprising of the sequence (5, 3, 4).
