+++
title = "is linear search faster than HashMap/HashSet for small sets?"
date = "2026-07-24"
description = ""
extra.contains_ai = "LLMs were used to generate benchmark test suites for this project."
taxonomies.tags = ["gamedev", "bevy", "rust"]
+++

I've heard from multiple source that when performance matters in small sets, like the hot-path in a game that runs every frame, it is often better to use a `Vec` or some sort of array to linearly search through a set instead of using a `HashSet` or `HashMap`.
The rationale is that while hash-based fetch and insert are constant time, they may be a _high_ constant time.
At some [small] size it must be faster to just iterate through the array instead of hashing a value.

So is it?

To test this out I made simple `VecSet` and `VecMap` implementations that implement a limited subset of the `HashSet` and `HashMap` API, namely `insert`, `remove`, `get`, and `iter`.
My implementation also went the route of storing `Option<T>` and `Option<(K,V)>` for `Set` and `Map` respectively, so I also added a `compact()` method to remove the `None` values from the storage.

* I got Claude to write some benchmarks comparing VecSet vs HashSet and VecMap vs HashMap at different data volumes, 4, 8, etc up to 1024 elements
* Data was always u64s for keys and values
* Comparing against Bevy's collections library which uses the non-standard [FixedHasher]
* Optimizations turned _on_
* All value are in nanoseconds.

# `VecSet` vs `HashSet`

> Winner is the fast implementation.

| Operation | Impl | 4 | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | Winner |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|
| **iterate** | VecSet | 3.5 | 6.1 | 11.2 | 21.6 | 42.3 | 89.3 | 172.2 | 337.9 | 670.2 | **Vec, every N (~2×)** |
| | HashSet | 6.0 | 10.6 | 20.7 | 40.9 | 81.2 | 161.5 | 322.8 | 652.3 | 1289.3 | |
| **build (`collect`)** | VecSet | 13.9 | 18.1 | 22.7 | 33.5 | 59.8 | 157.6 | 262.2 | 467.7 | 853.1 | **Vec, every N (~4–6×)** |
| | HashSet | 55.3 | 88.0 | 139.7 | 232.3 | 454.1 | 738.5 | 1346.0 | 2767.8 | 5281.8 | |
| **contains — hit** | VecSet | 2.6 | 3.7 | 6.0 | 10.7 | 19.9 | 38.2 | 81.5 | 155.1 | 302.3 | Vec ≲8, else **Hash** |
| | HashSet | 3.6 | 3.5 | 3.5 | 3.9 | 3.9 | 3.5 | 3.6 | 3.6 | 3.5 | |
| **contains — miss** | VecSet | 3.2 | 5.5 | 10.1 | 19.3 | 37.7 | 79.7 | 153.4 | 301.0 | 595.0 | **Hash, every N** |
| | HashSet | 2.7 | 2.9 | 2.9 | 2.7 | 2.9 | 2.9 | 2.7 | 2.7 | 2.7 | |
| **remove one** | VecSet | 24.7 | 38.7 | 44.1 | 60.8 | 112.6 | 219.6 | 376.6 | 685.5 | 1273.5 | Vec ≲8, else **Hash** |
| | HashSet | 25.9 | 35.6 | 38.4 | 45.8 | 42.9 | 45.4 | 46.3 | 45.8 | 50.1 | |
| **build (checked `insert`)** | VecSet | 26.0 | 136.9 | 244.4 | 596.0 | 1859.3 | 5916.5 | 21085 | 79255 | 308808 | Vec ≲48, else **Hash** (O(n²)) |
| | HashSet | 101.3 | 212.4 | 404.8 | 694.0 | 1305.3 | 2418.7 | 4608.5 | 8849.0 | 17437.6 | |

# `VecMap` vs `HashMap`

| Operation | Impl | 4 | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | Winner |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|
| **build (`collect`)** | VecMap | 15.1 | 17.6 | 25.9 | 42.8 | 129.7 | 201.8 | 353.1 | 651.1 | 1240.7 | **Vec, every N (~4×)** |
| | HashMap | 51.4 | 88.1 | 137.5 | 298.1 | 468.5 | 794.6 | 1428.0 | 2874.3 | 5613.2 | |
| **get — hit** | VecMap | 2.6 | 4.3 | 6.2 | 10.8 | 26.8 | 45.3 | 81.8 | 155.4 | 302.7 | Vec ≲8, else **Hash** |
| | HashMap | 4.2 | 4.5 | 4.2 | 4.2 | 4.3 | 4.2 | 4.2 | 4.2 | 4.2 | |
| **get — miss** | VecMap | 4.3 | 5.7 | 10.1 | 25.2 | 37.7 | 80.7 | 155.7 | 301.5 | 596.9 | **Hash, every N** |
| | HashMap | 3.5 | 3.5 | 3.5 | 3.5 | 3.5 | 3.5 | 3.5 | 3.5 | 3.5 | |

# jUsT uSe RaYoN

After the initial results I thought "surely we can parallelize the linear search right? SIMD something something" and did a side-quest adding a `par_*` version of the `VecSet`/`VecMap` types using rayon as the underlying engine.
Here are those results:

| Operation | Impl | 64 | 1024 | 16384 | 262144 | 1048576 |
|---|---|--:|--:|--:|--:|--:|
| **contains — miss** | VecSet seq | 57 | 892 | 14152 | 290913 | 1281114 |
| | VecSet par | 11330 | 17063 | 41296 | 308584 | 1169997 |
| | HashSet | 2.7 | 2.7 | 2.7 | 2.9 | 2.7 |
| **remove one** | VecSet seq | 72 | 787 | 11928 | 266622 | — |
| | VecSet par | 12485 | 18052 | 32334 | 244978 | — |
| | HashSet | 34 | 49 | 112 | 928 | — |

Rayon took _at least_ 11-12 µs which makes it _much_ slower at these small sizes.
As it turns out, Rayon has _hella_ overhead so for small sets like this the startup cost _way_ overshadows the compute time.

> Takeway: Rayon is great for processing _lots_ of data, like in batches.

# Takeaways

TODO


[FixedHasher]: https://docs.rs/bevy/0.19.0/bevy/platform/hash/struct.FixedHasher.html