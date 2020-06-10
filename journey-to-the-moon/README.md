# Journey to the Moon

[Hackerrank](https://www.hackerrank.com/challenges/journey-to-the-moon/problem)

The member states of the UN are planning to send  people to the moon. They want them to be from different countries. You will be given a list of pairs of astronaut ID's. Each pair is made of astronauts from the same country. Determine how many pairs of astronauts from different countries they can choose from.


Input Format

The first line contains two integers  and , the number of astronauts and the number of pairs.
Each of the next  lines contains  space-separated integers denoting astronaut ID's of two who share the same nationality.


```
Sample Input 0

5 3
0 1
2 3
0 4
Sample Output 0

6
```

Approach: Build a graph and traverse through it. Total combinations - Repeated ones.
nC2 is the number of combinations taken two at a time.
