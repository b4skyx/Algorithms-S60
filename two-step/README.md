# Count ways to reach the nth stair using steps 1 or 2.

The problem when solved using brute-force makes the user find all the possible ways by permutating through the step positons.

To solve it more efficiently, one could use the recursion and relate the problem to fibonacci series.

``Step(n) = 1 + Step(n-1) + Step(n-2)``

That is, he climbs the nth step either from n-1 or n-2 th step.
