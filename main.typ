#let theorem(body) = [*Theorem.* #body]
#let proof(body) = [_Proof._ #body #h(1fr) $square$]

= Boolean Algebra

== Hamming Weight
#theorem[The _hamming weight_ of an integer $N$ (represented in binary) can be computed in $O(log N)$ time.]\
#proof[
  Let $N=sum_(0 ⩽ k ⩽ n) a_k 2^k$; our goal is to compute $sum_k a_k$. Notice that it suffices to compute $sum_k a_k (2^k-1)$, and its difference with $N$ is our desired result.

  Recall that the _right shift_ bitwise operator has the following property:

  $
    N gt.double i = sum_(k ⩾ i) a_k 2^(k-i).
  $
  Since $2^k-1=sum_(1 < i ⩽ k) 2^(k-i)$, it follows that
  $
    sum_(i > 1) (N gt.double i)= sum_i sum_(k ⩾ i) a_k 2^(k-i)=sum_k a_k sum_(i ⩽ k) 2^(k-i)=sum_k a_k (2^k-1).
  $
  Therefore, the hamming weight of $N$ is

  $
    N-sum_(1 < i ⩽ n) (N gt.double i)
  $
  which can be computed in $O(log N)$ time.
]
