+++
title = "Independent Crypto Conclusion"

date = "2017-11-27"

description = "Sit back and ponder how much we've learned these past few weeks."

taxonomies.tags = [
    "independent crypto"
]
+++

{% note() %}
This is a part of a series of blog posts I wrote for an Independent Study on cryptography at Oregon State University.
To read all of the posts, check out the 'Independent Crypto' tag.
{% end %}

Wait, is it over already? It feels like we just started! Those 10 weeks always fly by, and this term was no exception.

Independent Crypto has been a thrill, an honor, and of course immensely educational.
It's been a blast to say the least.
The weekly check-ins with my mentor [Mike Rosulek](http://web.engr.oregonstate.edu/~rosulekm/), forcing myself to engage with academic papers and online lectures, and finally having complex topics like Garbled Circuits and Elliptic Curves "make sense", it was all very enjoyable.
I am very fortunate to have been given the opportunity to both create this course and take it.
Excited as I was before it began, it turned out even better than I could have hoped.

Let's reflect on the some of the things we learned...

## Elliptic Curve Cryptography

We learned that Elliptic Curves look like this:

![A straight forward ECC.](/images/independent-crypto/ecc-1.png)

and this:

![ECC with the line L illustrated](/images/independent-crypto/ecc-3.png)

We also learned that you can implement a version of the Diffie-hellman key exchange protocol by "adding" points on an Elliptic Curve over a finite field.

We also learned that despite how weird Elliptic Curve Cryptography sounds when you describe it, it can be used in very secure and efficient crypto.

## Memory Hard Functions

We learned that Memory Hard Functions (MHFs) are a solution to the arms race that is hash-cracking hardware.
Importantly, it relies on the fact that while there are specialized hashing processors, there is no specialized RAM for the same task (or any task really).

The goal of an MHF is to make it as hard (or harder) for an adversary to compute a given hash as it was for you, assuming you're running on non-specialized hardware and they have specialized hash-cracking hardware.

We learned that there are two types of MHF's: data-dependent and data-independent.
Data-dependent MFHs (dMHFs) have predictable memory usage patterns so they may be susceptible to cache-timing attacks.
Data-independent MHFs (iMHFs) are not susceptible to this attack as their memory patterns are not predictable.
While there are dMHFs in the wild, like scrypt which performs exceptionally well, there are not any any proven iMHFs in use.

We also learned that this is what it looks like to "Pebble an iMHF Directed Acyclic Graph":

![Animated DAG traversal.](/images/independent-crypto/dag-animated.gif)

We also learned that the way to attack an iMHF is by performing a breadth-first search on the graph, then once you hit a wall, fill in the
necessary nodes (pebbles) until you can compute the next node.
Much of the active research into iMHFs is in figuring out the best graphs, or types of graphs, to combat these kinds of feather/balloon attacks.

## Garbled Circuits

We learned that Garbled Circuits are a way of achieved two-party secure function evaluation.

Alice and Bob agree on a circuit (program) to garble.
One party encrypts the circuit by encrypting each logic-gate.
Both parties obfuscate their inputs and evaluate the garbled circuit.
This gives both parties the result of the original program without either party knowing the exact inputs.

This isn't a fool-proof security measure.
Sometimes it is good to ask the party garbling said circuits to create a few extras.
The evaluating party opens some of them to make sure they're on the up-and-up.
The other party evaluates the remaining circuits and verifies that the outputs are consistent.

We also learned that this game is surprisingly fun for only having **four levels**.

<iframe src="/garbled-circuits-game.html" height="400px" width="100%"></iframe>

Permalink: <http://elijah.run/garbled-circuits-game.html>

## Remote Timing Attacks

And now for something completely different!
We broke from the math and theory to focus on a problem involving real tangible bits!

Based on response timing differences an adversary can sometimes determine private information **like your OpenSSL Private Keys**.
This is scary, but the problem has mostly been fixed and now there are standards the crypto community follows to write code which is secure against these attacks.

If you're writing crypto and you know what you're doing, make sure you're using the defacto constant-time libraries for bit-wise comparisons, mathematical operations, and pretty much anything involving secrets.

## Miscellaneous

I personally gained a lot from this course.
I wrote annotated bibliographies, read academic papers, implemented a remote timing attack, and made a fun little game which was totally relevant and 100% not just an excuse to play around making games.

Despite sinking hours into this course I barely scratched the surface of every topic.
I've got *loads* more I could cover if I was inclined to do so.
I'm not saying I'm *going* to get a graduate degree, but if it's anything like this I'd be up for it.
