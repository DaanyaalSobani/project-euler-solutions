# Sum of Multiples of 3 and 5 Less Than N

## Problem

Need to sum up multiples of 3 and 5 less than $N$.

## Building Blocks

### Sum of first N natural numbers

$$\sum_{n=1}^{N} n = \frac{N(N+1)}{2}$$

### Sum restricted to multiples of 3

$$\sum_{n=1}^{N} n \cdot [n \text{ is multiple of } 3] = \sum_{k=1}^{K} 3k$$

where $K = \lfloor N/3 \rfloor$.

### Sum restricted to multiples of 5

$$\sum_{n=1}^{N} n \cdot [n \text{ is multiple of } 5] = \sum_{m=1}^{M} 5m$$

where $M = \lfloor N/5 \rfloor$.

## Combining with Inclusion–Exclusion

The sum of numbers that are multiples of 3 **or** 5 (subtracting multiples of 15 to avoid double-counting):

$$\sum_{k=1}^{K} 3k + \sum_{m=1}^{M} 5m - \underbrace{\sum_{\ell=1}^{L} 15\ell}_{\text{multiples of 15}}$$

## Closed Form

Applying the triangular-number formula to each sum:

$$= 3 \cdot \frac{K(K+1)}{2} + 5 \cdot \frac{M(M+1)}{2} - 15 \cdot \frac{L(L+1)}{2}$$

where, for multiples strictly less than $N$:

$$K = \lfloor (N-1)/3 \rfloor, \quad M = \lfloor (N-1)/5 \rfloor, \quad L = \lfloor (N-1)/15 \rfloor$$