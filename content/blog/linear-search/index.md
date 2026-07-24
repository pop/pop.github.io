+++
title = "is linear search faster than HashMap/HashSet for small sets?"
date = "2026-07-24"
description = ""
extra.contains_ai = "TODO"
taxonomies.tags = ["gamedev", "bevy", "rust"]
+++

I've heard from multiple source that when performance matters in small sets, like the hot-path in a game that runs every frame, it is often better to use a `Vec` or some sort of array to linearly search through a set instead of using a `HashSet` or `HashMap`.
The rationale is that while hash-based fetch and insert are constant time, they may be a _high_ constant time.
At some [small] size it must be faster to just iterate through the array instead of hashing a value.

So is it?

To test this out I made simple `VecSet` and `VecMap` implementations that implement a limited subset of the `HashSet` and `HashMap` API, namely `insert`, `remove`, `get`, and `iter`.
My implementation also went the route of storing `Option<T>` and `Option<(K,V)>` for `Set` and `Map` respectively, so I also added a `compact()` method to remove the `None` values from the storage.

Using Claude I 