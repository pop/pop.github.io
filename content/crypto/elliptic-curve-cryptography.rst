|ECC|
=====

:date: 2017-10-04
:status: published
:summary: The low-down on Elliptic Curve Cryptography.
:tags: Crytpo, Independent Crypto

.. note::

    This is a part of a series of blog posts I wrote for an Independent Study on cryptography at Oregon State University.
    To read all of the posts, check out the 'Independent Crypto' tag.

.. warning::

    This post is jumps around a bit.
    We'll start by showing how |ECC| works at a high level, then create a list of questions about how/why |ECC| works and how it is useful to cryptogrpahy.
    Once those questions are answered we will end with a recap.
    Hopefully we will zero in on what |ECs| are and what |ECC| is.

Diffie-Hellman key exchange ++
------------------------------

You find yourself day-dreaming during a walk around campus wondering if there is an alternative cryptography system to the very popular RSA.
You want something that has improved computational and network efficiency.
You want smaller keys that are harder to crack.
Could such a system exist?

You share this fantasy with a friend, you share all of your crypto fantasies with this friend, and they tell you that Elliptic Curve Cryptography is promising and it perfectly fits your needs.
... but how does it work?

Diffie-Hellman key exchange (a recap)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

To create a useful crypto out of |ECs| we need to implement `Diffie–Hellman key exchange`_ (DHKE).
Once we have DHKE we more or less have a valid crypto system which we can build upon to encrypt and decrypt private information.

The reader (you) is assumed to be familiar with DHKE.
While DHKE is fairly simple, it is not unforgettable, so here is quick reminder:

#. Alice and Bob agree on a public modulus (p) and a base (g).
#. Alice and Bob both choose secret integers (a and b).
#. Alice sends Bob |ga| (we call it A) and Bob sends Alice |gb| (we call it B).
#. Alice computes |Ba| and Bob computes |Ab|. These are equivalent (mod p). This is Alice and Bob's shared secret.

How do we use |ECs| get a similar 'shared secret'?

Elliptic Diffie-Hellman key exchange
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

At a (very) high level the algorithm is as follows:

#. Alice and Bob agree to use a given |EC| over a finite field, |EFp|, and a public point P |IN| |EFp|.
#. Alice chooses a secret integer |nA| and Bob choose secret integers |nB|.
#. Alice computes |QA| = |nA|\ P and Bob computes |QB| = |nB|\ P. These are the "Public Keys"
#. Alice sends Bob her public key, Bob send Alice his public key.
#. Alice computes |nA|\ |QB|, Bob computes |nB|\ |QA|.
#. The shared secret value is |nA|\ |QB| = |nA|\ (|nB|\ P) = |nB|\ (|nA|\ P) = |nB|\ |QA|

It looks similar to the given DHKE algorithm, and seems promising, but... how does it work?

|ECs| and |ECC| Q&A
-------------------

To answer that we are going to answer the following:

- What are |ECs|?
- What does an |EC| look like?
- What does it mean to multiply P by n?
- What about a finite field?
- How are the pubic keys used? Why are these a shared secret?
- Why is |ECC| useful?


What are |ECs|?
~~~~~~~~~~~~~~~

A |EC| is the set of solutions to an equation of the form

  |Y2| = |X3| + AX + B

What does an |EC| look like?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Two examples of |ECs| are as follows:

.. image:: /assets/images/independent-crypto/ecc-1.png
    :alt: A simple Elliptic Curve
    :align: center
    :width: 100%

and:

.. image:: /assets/images/independent-crypto/ecc-2.png
    :alt: Another simple Elliptic Curve
    :align: center
    :width: 100%

Adding P and Q
~~~~~~~~~~~~~~

Multiplication is *just* repeated addition.
Oh shoot we haven't said how "addition" happens on an |EC|.
Let's do that.

Addition is the process of drawing a line L between P and Q.
The third point that the line L intersects is point R.
When R is reflected over the X axis we call this R'.
The result of P |PLUS| Q (read: P 'plus' Q) is R'.

We can enumerate these steps as:

#. Take two points P and Q on the |EC| E.
#. Draw a line L which passes through these two points.
#. L should ultimately pass through *three* points: P, Q, and R.
#. Multiply the Y coordinate of R by -1, this is R'.
#. P |PLUS| Q = R'.

Here's a visualization of straight forward addition.

.. image:: /assets/images/independent-crypto/ecc-3.png
    :alt: Annotated curve E with points P, Q, R, R' and line L labeled.
    :align: center
    :width: 100%

You might think "What happens when P is tangent a point on E?"
In that case we say P = Q, so R = P |PLUS| P, or R = 2P.
It looks like this:

.. image:: /assets/images/independent-crypto/ecc-4.png
    :alt: Annotated curve E with points P, R, R' and line L labeled. P is tangent to the curve.
    :align: center
    :width: 100%

Wait a second, 2P looks like n*P which was one of the questions we had!
Don't worry, we'll get there soon.

That thing about Finite Fields
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

In practice we bound the curve over a field |Fp| with p |GTE| 3.
We input {1, 2, ..., p-1} as the value of X in E and select the results which are squares modulo 13.

For example:

  | E : |Y2| = |X3| + 3X + 8 over F\ :sub:`13`
  | X = 1
  | 1 + 3 + 8 = 12
  | 12 is a square (mod 13)

Repeating this gives us the set of points in E(F\ :sub:`13`):

  E(F\ :sub:`13`) = {O, (1,5), (1,8), (2,3), (2,10), (9,6), (9,7), (12,2), (12,11)}

In practice this bounds the graph of E and forces us to draw a strange modulus graph shown below:

.. image:: /assets/images/independent-crypto/ecc-5.gif
    :alt: Elliptic Curves illustrated where each point is a valid coordinate. There are no curves.
    :align: center
    :width: 100%

*Image source: A (relatively easy to understand) primer on elliptic curve cryptography* [2]_

Multiplying P by an integer with The Double-and-Add Algorithm
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

To "multiply" P by n we need to use the Double-and-Add Algorithm.
Here's how that looks:

0. Take a point P |IN| |EFp| and an integer n |GTE| 1.
1. Set Q = P and R = O.
2. Loop while n > 0.

   3. If n |EEEQ| 1 (mod 2), set R = R + Q
   4. Set Q = 2Q and n = floor(n/2).

5. Return the point R, which equals nP.

*Recall that the algorithm for finding point 2Q was covered in the above section* `Adding P and Q`_

What *is* the shared secret?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Let's review.
The shared secret is the second point |nA|\ |nB|\ P, which is a point on the public curve |EFp|.
This point can be used to encrypt information as it is a shared secret (necessary for DHKE).
How exactly it is used to encrypt information is left as an exercise for readers in charge of cryptographic implementation standards.

The reason this is a shared secret is because an adversary needs to solve the following |EC| |DLP|

    nP = |QA|

Which is a very hard problem, as mentioned in the next section.
 
An example of |ECC|
-------------------

This sounds good in theory, but let's give it a test drive.

Alice and Bob are given the following shared information:

  | p = 3851, E: |Y2| = |X3| + 324X + 1287, P = (920, 303) |IN| |EFpExample|

Alice and Bob choose their secret integers:

  | |nA| = 1194
  | |nB| = 1759

Alice and Bob then compute their public keys:

  | Alice computes |QA| = 1194P = (2067, 2178) |IN| |EFpExample|
  | Bob computes |QB| = 1759P = (3684, 3125) |IN| |EFpExample|

.. note::

    Remember that we use the Double-and-Add algorithm to compute |QA| and |QB|.
    This invloves iteratively computing the tangent line at a point, the intersection with E at that intersection, and reflecting that point over the X axis.

Alice and Bob trade public keys and calculate their shared secret:

  | Alice computes |nA|\ |QB| = 1194(3684, 3125) = (3347, 1242) |IN| |EFpExample|
  | Bob computes |nB|\ |QA| = 1759(2067, 2178) = (3347, 1242) |IN| |EFpExample|

Therefore (3347, 1242) is the shared secret.

Why |ECC| is useful
-------------------

While it is harder than simply multiplying mod p for Alice to compute her shared secret (which is the case in RSA), it is *even harder* for a malicious actor to figure out that same shared secret.
This point is best put by the source *A (relatively easy to understand) primer on elliptic curve cryptography* [2]_:

    You can compute how much energy is needed to break a cryptographic algorithm and compare that with how much water that energy could boil.
    This is a kind of a cryptographic carbon footprint.
    By this measure, breaking a 228-bit RSA key requires less energy than it takes to boil a teaspoon of water.
    Comparatively, breaking a 228-bit elliptic curve key requires enough energy to boil all the water on earth.
    For this level of security with RSA, you'd need a key with 2,380 bits.

So an |ECC| key can be one magnitude smaller in size and offer the same level of security as RSA.

We can put this in more concrete terms: the fastest algorithm to solve the |EC| |DLP|, which Elliptic DHKE security is built upon, in |EFp| takes |SQRT|\ p steps.
This is much more difficult than the 'vanilla' |DLP|.

Notes and edge cases
--------------------

|ECC|, much like the rest of Cryptography, deals heavily with `Number Theory`_.
Despite my best efforts most of the nitty-gritty Number Theory in this topic went *way* over my head.
As a result I didn't include much of that kind of stuff and instead focused on the things I *could* share and sound smart about.

Here are some other things about |ECC| I didn't cover that deserve more air time:

- The |EC| chosen must meet a special set of criteria; any old |EC| won't do. This was the cause of a cryptographic breach with |ECC| a few years ago that triggered doubts about |ECC| as a whole.
- Some primes cause solving the |EC| |DLP| for |EFp| to be easier than the |DLP|, these primes can be computed and should be avoided.
- If you want a deeper understanding of the theory of |ECs| (addition of points on these curves, etc) you should look into `algebraic geometry`_.

Annotated Bibliography
----------------------

An Introduction to Mathematical Cryptography [1]_
    The chapter in this textbook on |ECs| in Cryptography established the bedrock understanding of the topic of |ECC|.
    This ended up being the main resource for this post and offered a great median between "Regular Joe's guide to |ECC|" and "The graduate student's guide to |ECC|" which were my other two resources.
    It was also the source of all examples, which were very useful in gaining an intuitive understanding of the material.
    
A (relatively easy to understand) primer on elliptic curve cryptography [2]_
    This blog post was my *second* source and did a good job of taking the proofs and dense material in Intro to Math Cyrpto (above) and boiled it down to the important stuff.
    It drastically improved further readings of the original textbook and provided that great animated image of adding P |PLUS| Q in |EFp|.
    It didn't cover any of the Number Theory, but explained the historical context of |ECC|, roughly how/why it works, and did a good job of describing it's impact in our world today.

Cryptography: An Introduction [3]_
    This wasn't a resource I actually *used*, but I did read the chapter on Elliptic Curve Cryptography (chapter 2!).
    It gave me an appreciation for the previous two sources and some exposure to the other ways |ECs| can be taught.

Errata
------

.. [1]
    An Introduction to Mathematical Cryptography, 2008,
    Jeffery Hoffstein, Jill Pipher, Joseph H. Silverman,
    Springer Publishing, ISBN 978-0-387-77993-5

.. [2]
    A (relatively easy to understand) primer on elliptic curve cryptography,
    October 24, 2013, Nick Sullivan,
    Cloudflare blog, reposted on Ars Technica,
    https://arstechnica.com/information-technology/2013/10/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/

.. [3]
    Cryptography: An Introduction
    (Third Edition), May 19, 2016, Nigel Smart,
    https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf

.. |ECC|  replace:: Elliptic Curve Cryptography
.. |EC|   replace:: Elliptic Curve
.. |ECs|  replace:: Elliptic Curves
.. |PLUS| replace:: ⊕
.. |IN| replace:: ∈
.. |FORALL| replace:: for all
.. |Fp| replace:: F\ :sub:`p`
.. |QA| replace:: Q\ :sub:`A`
.. |QB| replace:: Q\ :sub:`B`
.. |Y2| replace:: Y\ :sup:`2`
.. |X3| replace:: X\ :sup:`3`
.. |y2| replace:: y\ :sup:`2`
.. |x3| replace:: x\ :sup:`3`
.. |ga| replace:: g\ :sup:`a` (mod p)
.. |gb| replace:: g\ :sup:`b` (mod p)
.. |Ab| replace:: A\ :sup:`b` (mod p)
.. |Ba| replace:: B\ :sup:`a` (mod p)
.. |EFp| replace:: E(F\ :sub:`p`)
.. |nA| replace:: n\ :sub:`A`
.. |nB| replace:: n\ :sub:`B`
.. |GTE| replace:: ≥
.. |EEEQ| replace:: ≡
.. |UNION| replace:: ∪
.. |SQRT| replace:: √
.. |DLP| replace:: Discrete Logarithm Problem
.. |EFpExample| replace::  E(F\ :sub:`3851`)
.. |EFpExample2| replace:: E(F\ :sub:`13`)
.. ∀

.. _Diffie–Hellman key exchange: https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange
.. _Number Theory: https://en.wikipedia.org/wiki/Number_theory
.. _algebraic geometry: https://en.wikipedia.org/wiki/Algebraic_geometry
