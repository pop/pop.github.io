+++
title = "Shuffle Good"

date = "2024-12-31"

description = "I wrote a shuffle algorithm... is it any good?"

taxonomies.tags = [
    "programming",
    "rust",
]

draft = true
+++

I programmed a Card game and at some point I needed to shuffle the deck of cards.

My first thought was "obviously Vec has a `shuffle()` method" but it does not!

Then I saw that the `rand` crate implements a `shuffle()` method, but I didn't want to pull in another crate just to shuffle.
Surely _I_ could write a shuffle function.

The algorithm I came up with looked something like this:

```rust
todo
```

This seemed to work initially, but at some point I _felt_ like the cards being drawn were consistently "samey".
I kept getting a lot of Blue and Green cards and not a lot of Reds.

So I said "Fine. I'll use the `rand` crate just so I'm _sure_ it's random and I'll come back to this later. Maybe it'll make a good blog post."

And now I'm here asking the question:

> If I write my own shuffle function, how do I know if it's any good?

Obviously we can start with `rand` crate's `shuffle` to start, comparing what we do to that.
Our goal isn't to _copy_ the rand shuffle, but to use it as a starting point.
Basically, take some list, run our `shuffle` on it, run the rand shuffle, and compare "shuffleness" of the two.
Ideally both would be roughly equally "shuffled".
But what the heck does that mean?
What does a good shuffle look like?

# Defining "A Good Shuffle"

Let's say we have an input list `L`.
We Shuffle that into list `S`.
For simplicity we assign the "value" of a card as it's order in `L` and we ignore the gameplay value.

> e.g., a deck `L` of 52 playing cards would not include information like diamond and king, but would instead rank each card 1..52.
> In a real use-case we would likely pair each card with it's original rank in `L` like `(1,{Two,Heart})`, `(52,{King,Spade})`.

## Idea 1: Diff Based Comparison

Maybe we could start by comparing the position of each element in `L` with each element in `S`.
That could work, but if we just shift all elements by 1 it would "look" like `S` is totally different from `L` when it's really just a _little_ different.
So let's dig deeper.

> Note: If you know of a more intelligent diff than just element-wise comparison let me know!

## Idea 2: Comparing Windows

We could scan windows of `S`, say scan 5 elements at a time in window `W`.
If each element in `W` is sorted then we score the algorithm down, if it's unsorted we score it up.
After going through all windows of `S` we tally up how unsorted the whole thing is.

This is immune from the "one element off" problem, but I could see based on the window size things are "still mostly sorted" but the algorithm gets a high score anyway.

## Idea 3: Adds and Subs

This does not build on the previous two ideas, but this feels like an interesting algorithm:
1. Iterate over every element in `S`.
2. Every even element we _add_ the value to a tally.
3. Every odd element we _subtract_ the value from a tally.

What's interesting here is that in the sorted list this would always return `0+/-1` -- e.g., `1+2-3+4-5+...`.
So ideally the more random the list is the farther from 0 we get.

## TODO: Academic research?

# Testing & Benchmarking

1. Start a crate
2. Add some benchmarks with rand
3. Add more benchmarks with your own thing
4. Compare one or two approaches
