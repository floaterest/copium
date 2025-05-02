# Bit Manipulation

## Hamming Weight


**Theorem.** The *hamming weight* of a number (represented as $n$ bits) can be computed in constant time and space.

*Proof.* Let $N=\sum_k a_k 2^k$ be a number represented in binary. Our goal is to compute

$$
\sum_k a_k.
$$

Notice that the problem then reduces to computing $S=\sum_k a_k (2^k-1)$ where $2^k-1$ is equal to $2^{k-1}+\cdots+2^2+2+1$.

Recall the *right shift* bitwise operator:

$$
N \gg i \stackrel{\text{def}}=\sum_{k\ge i} a_k 2^{k-i}.
$$

Therefore,

$$

$$
