# Marriage Problem

[Hackerearth](https://www.hackerearth.com/practice/data-structures/disjoint-data-strutures/basics-of-disjoint-data-structures/practice-problems/algorithm/marriage-problem/description/)
Monk has recently created a matrimonial site. X men and Y women registered there. As Monk has access to everyone's Facebook profile, he can see whether a person is a friend of another person or not. He doesn't want any two people who are in a single group to get married together. So, first we have relationships among men. Then, we have relationships among women. Finally we have relationships among men and women. Read the input format for more clarity. Now, Monk wants to calculate the total number of unique marriages he can set between men and women provided the conditions are followed.
Note - Two person are said to be in a group if they are friends directly or connected through a chain of mutual friends.


Approach: Using disjoint set unit, we can find the number of females not related to a male and add them for every male found.
