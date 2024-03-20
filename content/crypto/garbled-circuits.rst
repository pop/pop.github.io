Garbled Circuits
================

:date: 2017-10-30
:status: published
:summary: I know those words seperately but...
:tags: Crytpo, Independent Crypto

.. note::

    This is a part of a series of blog posts I wrote for an Independent Study on cryptography at Oregon State University.
    To read all of the posts, check out the 'Independent Crypto' tag.

Let's imagine you are a billionaire.
You want to know if you have more money than your billionaire friend Bob, but for some reason it's *very* faux pas to let anybody know how much money you have, even your good friend Bob.

But you're a billionaire!
You're not used to the phrase "that isn't possible".
In your frustration you try to figure out if you can use some of your billions to find a solution.

The first solution you come up with is Trusty Tina.
You tell Tina how much you make, Bob tells Tina how much he makes, and then Tina tells both of you who makes more.

The only problem is that Tina isn't *that* trustworthy, her parent's just named her that.
Like... you wouldn't trust her with your life or anything.
You could pay her to keep your assets secret but Bob might pay her a bit more reveal your number to him.
Tina reminds you of this so you have to pay her *more* to keep the secret, and eventually you have an arms-race type situation at hand.
With your cunning accountant skills you can already tell that Tina might be more trouble than she's worth.

If you won't tell Bob directly, and you can't depend on Manipulative Tina, is there *any* way to determine out who has more money?

Yes.

2-Party Secure Function Evaluation
----------------------------------

The problem above is the Millionaires problem [9]_ and it is solved by the use of 2-party secure function evaluation (SFE).
The general idea [4]_ is that you have a function ``f`` which takes as input ``x`` and ``y``.
This function is garbled by one of the two parties into ``f'`` and the inputs are garbled into ``x'`` and ``y'`` by each of the parties.
``f'(x', y') == f(x,y)`` but does not leak any of the inputs, because ``x`` and ``y`` were obfuscated.

In other words...

1. Alice and Bob agree on a function (i.e., circuit) ``f(a,b)``.
#. Alice garbles the function (*cough* circuit) ``f`` and her input ``a``.
#. Alice sends ``f'`` and ``a'`` to Bob.
#. Bob gets his input ``b`` garbled into ``b'``.
#. Bob evaluates ``f'(a', b')``.
#. This reveals the result of ``f(a,b)`` to Bob, but does not reveal Alice's input.

.. note::

  The first half of this post focuses on honest garbled circuit uses, meaning both parties are acting honestly (and don't pull any fast-ones).

  The latter portion focuses on problems with that 'vanilla' garbled circuit implementation and potential solutions.

Garbling a gate
---------------

How does Alice actually... 'garble' a circuit?
It sounds kinda dirty.

Each gate (OR, AND, XOR, etc) has two inputs.
Stick with me.
Each input is encrypted.
Keeping up?
So you need a *key* to use *each gate*.
It gets better.
But if you have the keys, you don't know which key corresponds with a ``1`` or a ``0`` so you can compute a function without knowing the actual values you put in.
Whoa.

Let's use the OR gate as an example.

Remember truth-tables for OR?
Here's a reminder:

===== === ===
 OR    0   1
===== === ===
**0**  0   1
**1**  1   1
===== === ===

This table is going to be important for Alice's part of this dance.

Alice's circuit & input
~~~~~~~~~~~~~~~~~~~~~~~

Alice definitely does the heavy lifting in this transaction.

1. Alice generates 4 keys ``Kx0``, ``Kx1``, ``Ky0``, and ``Ky1``.
2. Alice creates four variables corresponding with the four values in the OR table:

===== =========== ===========
 OR    0           1
===== =========== ===========
**0** ``B00`` = 0 ``B01`` = 1
**1** ``B10`` = 1 ``B11`` = 1
===== =========== ===========

3. Each box is encrypted with the two keys:

===== ==================== =====================
 OR    0                    1
===== ==================== =====================
**0** ``E(Kx0||Ky0, B00)`` ``E(Kx0||Ky1, B01)``
**1** ``E(Kx1||Ky0, B10)`` ``E(Kx1||Ky1, B11)``
===== ==================== =====================

4. Alice sends these ciphertexts (unordered) to Bob.


Bob's input
~~~~~~~~~~~

5. Bob gets Alice's input, key ``KxA``, from Alice.

6. Bob uses oblivious transfer [5]_ to get his input ``KyB``.

7. With these two keys Bob is able to process the circuit (an OR gate).

Bob has enough information to get one of the four possible outputs of the circuit, but doesn't know if Alice's input is a 1 or a 0.

Importantly, while Bob can share the output of the circuit, he should **not** share his input.
That would make using OT (step 6) obtuse.

Extending the garbled gate
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. image:: /assets/images/independent-crypto/garbled-circuit.jpg
    :align: center
    :width: 100%
    :alt: Garbled circuit example diagram.

.. note::

    The UTF-8 Padlock symbol doesn't render on my browser because I seem to have gone back in the time to the late 90s.
    Being stuck in the past, we have to comprmise.
    The ⛨ symbol is meant to represent a lock and the ⚿ represents a key.

Multiple gates can be connected together to build more complicated circuits.
One important difference is that while each intermediate circuit still has four cipher-texts, for the four outcomes of a truth-table, those decrypt to a *key* and not a 1 or 0.
The only gates which decrypt to a plain-text of 0 or 1 are output gates, not the intermediate gates.

.. note::

  *PSST* Check out the end of this post for a **GAME**!

Problems with garbled circuits
------------------------------

There are a few important flaws in the *security* of garbled circuits as they have been described.
The first is that although Alice and Bob agree on a circuit to garble there is no guarantee that the circuit one is evaluating (if you're Bob) is the circuit you agreed upon.

For example:

1. Alice and Bob 'agree' on a function ``f(a,b)``.
#. Alice creates her own function ``g(a,b)``.
#. Alice garbles ``g`` and her input ``a`` and sends it to Bob as ``f'`` and ``a'``.
#. Bob evaluates ``g'(a',b')`` and reveals the output to Alice. Alice now knows something other than than Bob agreed to.

Improvements on garbled circuit security
----------------------------------------

To prevent the above adversarial attack we do something called "Cut-and-Choose".
This is when Bob checks Alice's work to make sure she's not cheating.

Remember that Alice and Bob agreed on a given circuit.

1. Alice generates M garbled circuits for the agreed upon function where M > 1.
#. All secrets for a randomly chosen N circuits are revealed where 1 ≤ N < M, the circuits are "opened".
#. Bob selects one of the remaining M-N circuits to evaluate as outlined earlier.

This ensures that Alice is not nefarious to some statistical certainty.
She had control over how the circuits were garbled but she does not have control over which are revealed or evaluated.
If she made one (or two or three...) nefarious circuits that bad behavior is *probably* revealed in step 2, if all the checked circuits are good Alice is *probably* being honest.

This doesn't break garbled circuits for the following reasons:

- While Alice reveals the secrets of the N circuits, she doesn't reveal anything about her input. We are only un-garbling the circuit not the inputs (revealing all possible inputs, not Alice's).
- We're not un-garbling the M-N circuits which may be evaluated, so those are still secret.

As M grows and N approaches M this method gets more secure at the cost of computation cycles and bandwidth in transferring the garbled circuits.

The "Free XOR" Optimization
---------------------------

I'm definitely not a circuits person.
You can show me a circuit diagram and I'll say "Yep, that's a circuit. What's it do?"
I couldn't even even identify which gate is which without Wikipedia.

I was told during my research for this post that XOR gates are very popular with garbled circuit design, and more broadly circuit design in general.
This was shared to me in the form of a cryptic hint so I figured I'd investigate and share my findings here.

As it turns out the Wikipedia page notes that this XOR optimization exists and even cites the original paper published on the topic. [6]_

The jist of this optimization is that one can very efficiently garble an XOR gate such that the output of the gate is encoded as the XOR of the keys used to unlock the gate and some known global constant.
This is in contrast with the implementations discussed in the beginning where each gate had to be decrypted with two cipher-texts and revealed another key.

Basically using XOR, which is pretty fast, we can avoid generating four keys per gate and instead craft 1 key which is produced as the result of 'unlocking' a gate.

Put a bit more formally:

  Given a gate G with input wires A and B and output wire C and a random string R, the garbled gate is obtained by XORing the garbled gates inputs C\ :sup:`1` = C\ :sup:`0` |XOR| R:

   | C\ :sup:`0` = A\ :sup:`0` |XOR| B\ :sup:`0` = (A\ :sup:`0` |XOR| R ) |XOR| (B\ :sup:`0` |XOR| R) = A\ :sup:`1` |XOR| B\ :sup:`1`
   | C\ :sup:`1` = C\ :sup:`0` |XOR| R = A\ :sup:`0` (B\ :sup:`0` |XOR| R ) = A\ :sup:`0` |XOR| B\ :sup:`1` = (A\ :sup:`0` |XOR| R) |XOR| B\ :sup:`0` = A\ :sup:`1` |XOR| B\ :sup:`0`

.. note::

  LETTER\ :sup:`{0,1}` is short-hand for the True or False output of the given gate.

This isn't super intuitive, and honestly I just put those equations up there to prove that I read a paper about this.

The main takeaway is that 'free XOR' saves us computation generating and processing cryptographic keys by simply performing the XOR operation.
This optimization is so powerful that using *mostly* XOR gates makes garbled circuits notably faster and more useful for secure computation. [8]_

Annotated Bibliography
----------------------

Foundations of Garbled Circuits [2]_
    This is by far the most thorough academic source I have.
    If I had a better foundation in academic reading this might be the perfect paper but most of it went way over my head.
    That said the overview of each section was fairly human-readable and gave me a good rough overview for many of the topics covered in this post.

A Brief History of Practical Garbled Circuits [8]_
    This was the first source I checked out to get a feel for how difficult garbled circuits are as a topic.
    It quickly glanced at the basics of garbled circuits and then quickly went into the optimizations on garbled circuits.
    This was overwhelming, but as I started to learn more about garbled circuits and filled in the knowledge gaps it gained significant value.

    It's a great talk about Garbled Circuits which wasn't ideal for beginners, but did give me a good breadth of the topic and what I could dive into.

Improved garbled Circuit: Free XOR Gates and Applications [6]_
    This paper was useful in giving me an academic preview of the XOR optimization in Garbled Circuits.
    I quickly started looking at the many other papers referenced by this one, kind of like following down the Wikipedia wormhole, but with more PDFs and less pictures.

SFE: Yao's Garbled Circuit [1]_
    This slide-deck was very useful as a reference for basic GCs and securing GC's with cut-and-choose.
    It wasn't a great initial source for this material, but it was useful *after* I had a basic understanding of a topic to solidify it with pretty pictures and Comic Sans.

Mike Rosulek on Stack Exchange [3]_ [7]_
    This is more of a shout-out than a citation.
    Mike Rosulek's posts on crypto.stackexchange.com were very helpful in breaking down core concepts like what garbled circuits are and why XOR is "free".
    They also provided a good list of further reading which was helpful in addition to the resources provided in the syllabus.

Errata
------

.. raw:: html

  <iframe src="/garbled-circuits-game.html" height="400px" width="100%"></iframe>

.. note::

  Yes, the name is a misnomer.
  The goal is to *evaluate* a garbled circuit, but that just doesn't roll off the tongue the same.

.. [1]
  SFE: Yao's Garbled Circuit,
  Published by engr.illinois.edu,
  for the course CS 598, Fall 2009.
  https://courses.engr.illinois.edu/cs598man/fa2009/slides/ac-f09-lect16-yao.pdf

.. [2]
  Foundations of Garbled Circuits,
  Written by Mihir Bellare, Viet tung Hoang, and Phillip Rogaway,
  Published October, 2012.
  https://eprint.iacr.org/2012/265.pdf

.. [3]
  What exactly is a "garbled circuit"?
  Asked by user Ella Rose,
  Answered by user Mikero on on July 27, 2016.
  https://crypto.stackexchange.com/a/37993

.. [6]
  Improved garbled Circuit: Free XOR Gates and Applications,
  Written by Vladimir Kolesnikov and Thomas Shneider,
  Published July 2008.
  http://www.cs.toronto.edu/~vlad/papers/XOR_ICALP08.pdf

.. [7]
  Why XOR and NOT is free in garbled circuits
  Asked by user Jason,
  Answered by user Mikero on February 28, 2017.
  https://crypto.stackexchange.com/a/44278

.. [8]
  A Brief History of Practical Garbled Circuit Optimizations,
  Presented by Mike Rosulek,
  Published by the Simons Institute,
  June 15, 2015.
  https://youtu.be/FTxh908u9y8

.. [4]
  To completely level with you, it's been anecdotally proven that there is at least 1 definition of Garbled Circuits for each paper on the topic.

.. [5]
  Oblivious Transfer has been described to me as:

  - Alice sends two possible options to a box labeled OT.
  - Bob sends a choice to the box labeled OT.
  - Bob gets back one of the two options, without knowledge of the other.
  - Alice does not know which option Bob got.

  This is a cryptographic primitive which is very useful for tasks like generating Bob's input to the garbled circuit ``f'``.

.. [9]
  The original problem was developed in the 80's.
  This post adjusts the scenario for inflation.

.. |XOR| replace:: ⊕

.. garbled circuits are kinda like monads.
   When everybody hears about them they write a blogpost (paper) explaining them in their own unique 'intuitive' way.

.. Honestly, Yao got out of this pretty easy.
   He said "Here's a thing, but I'm just gonna say it, not prove it" and everybody was like "Oh shit that could be a thing."
   If he was wrong somebody would just say "Oh that's not really a thing." and inevitably *not* publish because that's boring.
   If he's right (he is) he's like Euler; jotting things down in the margins for other people to prove.
   At least that's the story I'm reading.
