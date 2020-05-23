# GCD using euclidean algorithm.

This algorithm is one of the efficient algorithms to find the Greatest Common Divider or the Highest Common Factor of the two numners.

In this algorithm, we subtract the smaller number from larger number, which doesn't changes the gcd.

for exampe in
```
6, 3

6-3 = 0

3, 0
```


This subtraction instead of repetition can be calculated efficiently by modulous of two numbers.

Hence, if two numbers are a and b,
(a = b, b= a%b) till b = 0.
