#import "preamble.typ":*
= Sliding Window

Many problems in the format "given an array, count the number of subarrays that satisfies certain property" can be solved using the _sliding window_ technique in $O(n)$ time--despite there are $O(n^2)$ many subarrays. These algorithms are often _online_ algorithms and their behavior is very similar to greedy algorithms. Therefore, we can easily generalise them to infinite arrays.

Given an array $a=(a_0,a_1,⋯) in A^ω$ and indices $i,j < ω$, let $a[i..j]$ denote the _subarray_ $(a_i,⋯,a_j)$. The sliding window technique usually follow these steps:
+ Given some definition of a "valid subarray $a[i..j]$", let $c in NN$ be the count of all valid subarrays, write $c=sum_(j < ω) c_j$ where $c_j$ is the count of those that ends at $j$ (i.e. number of $i ⩽ j$ such that $a[i..j]$ is valid).
+ For each $j < ω$, define $i=i_j < j$ such that $c_j$ can be determined with respect to $i$ and $j$. This usually implies that $a[i..j]$ is valid and sufficient to deduce all valid subarrays ending at $j$. Computing $i_j$ might require some data structure where an incremental update from the $j$th iteration to the $(j+1)$th can be done in constant time.

Following the steps above, we call $(i,j)$ a _window_. Each time we increment $j$, we say that the window _expands_; and each time we compute $i$ (usually $i_j < i_(j+1)$), we say that the window _shrinks_.

For convenience, we will say "$(i,j)$ is valid" to mean that the subarray $a[i..j]$ is what we want to count. We shall use $a_i$ and $a(i)$ to refer to the $i$th element of $a$. 

== Count Subarrays With Fixed Bounds

#link("https://leetcode.com/problems/count-subarrays-with-fixed-bounds")

*Problem.* Given an array $a in ZZ^ω$ and $m,M in ZZ$, count the number of subarrays $a[i..j]$ whose minimum is $m$ and maximum is $M$.

_Proof._ Let $c$ be the number of $(i,j)$ such that

$
  min_(i ⩽ k ⩽ j) a_k=m "and" max_(i ⩽ k ⩽ j) a_k=M.
$
Write $c=sum_(j < ω) c_j$ where $c_j$ is the number of $i$'s such that $(i,j)$ is valid.

Fix right endpoint $j$, define the following left endpoints:
+ let $i^* ⩽ j$ be the largest index such that $a(i^*) <m$ or $a(i^*) > M$;
+ let $i_m ⩽ j$ be the largest index such that $a(i_m)=m$;
+ let $i_M ⩽ j$ be the largest index such that $a(i_M)=M$.

Notice that if $i_m$ or $i_M$ does not exist, then $c_j=0$ as no subarray ending at $j$ is valid. If $i^*$ does not exist, then all subarrays ending at $j$ is valid: thus $c_j=i+1$ in this case. Let $i=min{i_m,i_M}$. If $i^* < i$, then $(i',j)$ is valid if and only if $i^* < i' ⩽ i$, hence $c_j=i-i^*$. Otherwise (when $i < i^*$), $c_j=0$. #h(1fr) $square$

== Subarrays with K Different Integers

#link("https://leetcode.com/problems/subarrays-with-k-different-integers/")

*Problem.* Given an array $a in A^ω$ and $k in NN$, count the number of subarrays with $k$ unique elements.

_Proof._ Let $c$ be the answer, and write $c=sum_j c_j$ where

$
c_j=|{i:a[i..j] "has" k "unique elements"}|.
$

Fix $j$, let

+ $i_"max"$ be the largest index such that $a[i_"max"..j]$ has exactly $k$ elements;
+ $i_"min"$ be the largest index such that $a[i_"min"..j]$ has exactly $k+1$ elements.

If $i_"max"$ doesn't exist, then $c_j=0$. Otherwise, if $i_"min"$ doesn't, then $c_j=i_"max"+1$ as any subarray ending at $j$ would have exactly $k$ elements in this case.

Notice that $i_"min" ⩽ i_"max" ⩽ j$, and any subarray (ending at $j$) that starts before or at $i_"min"$ has more than $k$ elements, and any subarray that starts after $i_"max"$ has less than $k$ elements. Therefore $(i,j)$ is valid iff $i_"min" < i ⩽ i_"max"$. Thus $c_j=i_"max"-i_"min"$. #h(1fr) $square$

