<!--
## DP
- [`abc211_c.rs`](./atcoder.jp/abc211/abc211_c.rs)
-->

# Algorithms

## Binary Search
> use [`binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search) instead

```rs
fn binary_search<T: Ord>(slice: &[T], target: T) -> Result<usize, usize> {
    // slice.binary_search(&target)
    let (mut left, mut right) = (0, slice.len());
    while left < right {
        let mid = (left + right) / 2;
        if slice[mid] < target {
            left = mid + 1;
        } else if slice[mid] > target {
            right = mid;
        } else {
            return Ok(mid);
        }
    }
    Err(left)
}
```


## Lower and Upper Bound
> use [`binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search) (if `x` is not found, it returns index of smallest item in slice that is bigger than `x`)

input
- $(a_i)_{i=1}^N$ as a **increasing** sequence of $\mathbb Z$
- $L,U\in\mathbb Z$ st $1\le L\le U\le N$

find $r-l$ where $\forall j\in[l,r)\quad L\le a_j\le U$
- i.e. size of subsequence bounded by $L$ and $U$


```rs
fn bounded(a: &[i32], lower: i32, upper: i32) -> usize {
    use std::convert::identity;
    // lower <= x <= upper ==>  a[left] <= x < a[right]
    let left = a.binary_search(&lower).unwrap_or_else(identity);
    // +1 because if upper == a[i] for some i,
    // then we want `right` to be i + 1 to satisfy j < r
    let right = a.binary_search(&(upper + 1)).unwrap_or_else(identity);
    right - left
}
```

see [`abc192_e.rs`](./atcoder.jp/abc192/abc192_e.rs#L181)


# Number Theory

## Ceil Integer Division

for $p\in\mathbb N$ and $q\in\mathbb Z^+$

$$
\lceil p/q\rceil=\lfloor(p + q - 1)/q\rfloor
$$

# Graph Theory

## Dijkstra
> use [`BinaryHeap`](https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples)


```rs
fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize, end: usize) -> Option<usize> {
    //! find shortest path from `start` to `end`

    // set all weights (from start) to MAX
    let mut w: Vec<usize> = vec![!0; graph.len()];
    // w(start, start) is 0
    w[start] = 0;
    let mut heap = BinaryHeap::from([Reverse((0, start))]);
    // check the node with the lowest weight first (min-heap)
    while let Some(Reverse((wu, u))) = heap.pop() {
        // reach destination (shortest)
        if u == end { return Some(wu); }
        // current path is not the shortest
        if wu > w[u] { continue; }
        // w(start, v) = w(start, u) + w(u, v)
        graph[u].iter().map(|&(v, uv)| (v, wu + uv)).for_each(|(v, wv)| {
            // if new weight is less
            if wv < w[v] {
                heap.push(Reverse((wv, v)));
                w[v] = wv;
            }
        });
    }
    None
}
```
