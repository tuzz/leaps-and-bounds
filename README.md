## Leaps and Bounds

A tool to find superpermutation bounds.

```
cargo run --release
```

There is more high-level explanation
[here](https://github.com/tuzz/leaps-and-bounds/blob/master/src/ui/mod.rs#L6).

## Overview

This project was an attempt to improve upon the results presented in [this blog
post](http://www.njohnston.ca/2014/08/all-minimal-superpermutations-on-five-symbols-have-been-found/).

The project was somewhat successful as it is able to discover bounds faster than
the competing C implementation, but this speed up isn't sufficient to make
enough of a dent in the problem to find a minimal superpermutation for six
symbols.

## Key ideas

Below is a brief summary of the ideas explored in this project.

---

**1) We can use a BitSet to speed up checking which permutations have been seen**

As we explore the search space, each 'candidate' string can be stored with a
BitSet of the permutations it contains. This allows us to check in constant time
whether we've seen a permutation before when the candidate is expanded into its
child strings. This is faster than having the re-scan the entire string, which
could be many hundreds of characters in length.

To achieve this, the implementation uses the [Lehmer crate](https://github.com/tuzz/lehmer)
as a bijection between permutations and decimal numbers. Each candidate is
stored with a BitSet of length N! bits.

---

**2) We use a best-first search to prioritize which strings are expanded next**

Rather than treat all strings equally, we instead prioritize which strings are
expanded based on how many wasted symbols they contain. This includes a
projection of how many 'future' wasted symbols there will be for strings like
`123454`, which guarantees the next two symbols will be wasted, regardless of
what they are because the `4` occurs mid-way through the tail of the string.

To achieve this, the implementation uses the [Bucket Queue](https://github.com/tuzz/bucket_queue)
crate as a priority queue to efficiently add and dequeue items in near-constant
time.

---

**3) We extend the use of bounds to speed up the search**

Once we know that a maximum of P permutations can fit into a string that wastes
W symbols, we can use this knowledge to prune regions of the search space. This
is the key idea introduced in the blog post linked above, but we extend this in
a couple of ways:

- We can determine an 'upper bound' on the 'lower bound' of the maximum number
of permutations that fit into a string that waste W symbols. This means we can
determine when it's no longer possible to improve over a string we've already
found and can 'short-circuit' the search, moving onto its next phase.

- We can move onto the next phase of the search (for W+1 wasted symbols) without
restarting from the beginning by holding onto regions that were previously
pruned and restoring them if the current phase needs to explore them. See
`Frontier#prune` and `Frontier#unprune` for more detail about how this works.

---

**4) We offload/onload regions to/from disk if memory fills up**

One disadvantage of this approach is that memory fills up quickly as we need to
hold on to all pruned regions in case they're needed again in a subsequent
search phase. To combat this (somewhat) we track the memory usage of the program
and offload regions to disk when memory fills up. When the search needs to
expand candidates that are stored on disk, they are onloaded back into memory
again. These files can be compressed if desired to save disk space.

## Closing remarks

I worked on this project as a hobby and I'm pleased with the progress I made.
Unfortunately, the program realistically won't be able to exhaust the entire
search space and find the shortest superpermutation for N=6 as it's simply too
large, even with the improvements mentioned above. Left to run for several days,
it consumes many terrabytes of disk space and spends the majority of its time
moving data back and forth between memory and disk, which is a shame.

This has inspired me to further my knowledge of search algorithms and work on
these follow-up projects that take different approaches to solving the same
problem:

- https://github.com/tuzz/supermutation
- https://github.com/tuzz/supersat

I'm sorry this write-up is brief and there is no formal presentation of these
ideas.

If you have any questions, please DM [me on twitter](https://twitter.com/chrispatuzzo). Thanks.
