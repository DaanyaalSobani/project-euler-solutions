# Problem

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20 ?

## Initial thoughts

$$
M_n = \{ k \in \mathbb{Z}^+ \mid k \bmod i = 0 \; \forall \, i \in \{1, 2, \ldots, n\} \}
$$
Let $M_n$ be the set of numbers the are cleanly divisible by the numerbs from $1$ to $n$.

We want to know what $\min(M_{20})$ is. We note that
$$
n! \in M_n
$$
since
$$
n! = 1 \times 2 \times \cdots \times n
$$
and this number must be divisible by every number from $1$ to $n$.

I'm not sure that this will help us in finding the minimum but it's interesting.
