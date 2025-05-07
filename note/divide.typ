#import "preamble.typ": *
= Divide and Conquer

_Divide and conquer_ (D&C) refers to a class of recursive algorithms that split a problem of input size $n$ into $a$ many subproblems of size $n"/"b$. The solutions to the subproblems are then combined in $O(n^d)$ time to give a solution to the original problem.

Let $T(n)$ be the running time of a D&C algorithm, and write $T(n)=a T(n"/"b)+O(n^d)$. The #link("https://en.wikipedia.org/wiki/Master_theorem_(analysis_of_algorithms)")[Master theorem] says that

$
  T(n)=cases(
    Theta(n^d) & "if" a < b^d,
    Theta(n^d log n) & "if" a=b^d,
    Theta(n^(log a)) & "if" a > b^d
  ).
$
For instance, binary search has running time $T(n)≝ T(n"/"2) ==> T(n)=O(log n)$.

== Median of Two Sorted Arrays

#link("https://leetcode.com/problems/median-of-two-sorted-arrays/")

*Theorem.* Given two sorted arrays $A in ZZ^n$ and $B in ZZ^m$, the median of $C=A ⊔ B$ can be computed in $O(log min(n, m))$ time.

#proof[
  WLOG assume $n ⩽ m$, we describe a D&C algorithm that does binary search on ${0,⋯,2n}$, thus taking $O(log n)$ time.

  For index $i$, define a _cut_ of $A$ _at_ $i$ to be a pair of subarrays $(A_L,A_R)$ where
  $
  A_L=(a_0,⋯,a_i) "and" A_R=(a_i,⋯,a_(n-1)).
  $
  Similarly, a _cut_ of $A$ _between_ $i-1$ and $i$ is the pair $(A_L,A_R)$
  $
  A_L=(a_0,⋯,a_(i-1)) "and" A_R=(a_i, ⋯,a_(n-1)).
  $

  Given a cut $(A_L,A_R)$, let $a_L=max A_L$ and $a_R=min A_R$ (or $-∞$ and $∞$ if a subarray is empty). Notice $a_L ⩽ a_R$.

  For convenience, let $N=2n+1$ and define $f_A :N harpoon ZZ$ where for each $i < n$
  $
  f_A (2i) &= bot\
  f_A (2i+1) &= a_i
  $
  Therefore, we can define a "cut" of $f_A$ is a similar way:
  - Let the cut of $f_A$ at $k=2i$  be the cut of $A$ between $i-1$ and $i$.
  - Let the cut of $f_A$ at $k=2i+1$ be the cut of $A$ at $i$.
Notice in both cases, $a_L$ is the $floor((k-1)/2)$th element of $A$ with $|A_L|=ceil(k"/"2)$, and $a_R$ is the $floor(k/2)$th element of $A$ with $|A_R|=n-floor(k"/"2)$.

#let med = math.op("med")

To find median of $C ≝ A ⊔ B$ (denoted as $med C$), we need to look for a cut $(C_L,C_R)$ of $C$ such that $|C_L|=|C_R|$. This cut can be induced from a cut $(A_L,A_R)$ of $f_A$ at some $k$ and the cut $(B_L,B_R)$ of $f_B$ at $k'=n+m-k$ satisfying $a_L ⩽ b_R$ and $b_L ⩽ a_R$. The median is then 
$
med C=(max C_L+min C_R)/2=(max{a_L,b_L}+min{a_R,b_R})/2.
$

The inequalities implies that $(C_L=A_L ⊔ B_L,C_R=A_L ⊔ B_R)$ is a valid cut of $C$. The choice of $k'$ implies that, since

$
n+m=k+(n+m-k)=ceil(k"/"2)+floor(k"/"2)+ceil((n+m-k)/2)+floor((n+m-k)/2),
$
it follows that

$
|C_L|=|A_L|+|B_L| &= ceil(k"/"2)+ceil((n+m-k)/2)\
&= n-floor(k"/"2)+m-floor((n+m-k)/2)\
&= |A_R|+|B_R|=|C_R|.
$

We use binary search to find such $k$. Let $i_0=0$ and $j_0=2n$, we know that $i_0 ⩽ k ⩽ j_0$. Let $k_0=(0+2n)"/"2$ be a guess, and define $k'_0=n+m-k_0$. Let $(a_L,a_R)$ be the $floor((k_0-1)/2)$th and $floor(k_0/2)$th element of $A$, and $(b_L,b_R)$ be the $floor((k'_0-1)/2)$th and $floor(k'_0/2)$th element of $B$. 
- If $a_L > b_R$, then $k_0$ is too large, meaning $k$ is between $i_1=i_0$ and $j_1=k_0-1$.
- If $a_R < b_L$, then $k_0$ is too small, meaning $k$ is between $i_1=k_0+1$ and $j_1=j_0$.
]


