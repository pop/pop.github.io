Remote Timing Attacks
=====================

:date: 2017-11-20
:status: published
:summary: Or yet another reason you should never roll your own crypto.
:tags: Crytpo, Independent Crypto

.. note::

    This is a part of a series of blog posts I wrote for an Independent Study on cryptography at Oregon State University.
    To read all of the posts, check out the 'Independent Crypto' tag.

Fade in.

It is 2002.

You are a Linux system administrator.
You and thousands of other admins are running OpenSSL on you battle tested Linux servers.
You trust that your data is transfered securely from host to host because... why wouldn't it be?!
OpenSSL makes things secure.
Duh.

A few months into running that server you figure out that your private keys have been compromised!
They were stolen somehow but you can't figure out what happened.
You check the logs to see if somebody hacked into your system, but nothing obvious catches your eye.

Combing through the logs you do see an IP address that tried (and failed) hundreds of thousands of times to authenticate with your OpenSSL server.
It attempted authentication over and over and failed relentlessly until suddenly, after about two days, it stopped.
Weird.

A year later you read an academic paper from Stanford. [3]_
It clicks.
You've you've been Remote Timing Attacked!

Definition
----------

Remote Timing Attacks are a special brand of Side Channel Attack where adversaries use differences in response times to determine private information.
Creepy I know.

A lot of code and Statistics\ :sup:`TM` goes into figuring out secret information based on this, but let's start small.
Take the following comparison which lives in a hypothetical SSL library:

.. code:: python

    if length(recieved_key) != PRIVATE_KEY_LENGTH:
        return KeyLengthError

    if recieved_key == expected_key:
        return Thing
    else:
        return UnknownKeyError

This might seem innocent enough but as it turns out this leaks information!
The ``==`` operator does not *always* take the same amount of time to return a response.
In fact, this operator tends to compare two inputs bit-by-bit.
When it finds a difference it short-circuits and returns True or False.
If ``recieved_key`` shares the first N bits with ``expected_key``, the program will return slightly sooner than if they only shared the first N-1 bits.

What's the timing difference if it's just returning one or two cycles earlier; does it really matter?
As it turns out, there is *enough* of a difference to break security and enable an adversary to decrypt entire private keys! [3]_
This is put well by an article on chosenplaintext.ca: [4]_

    Now, it may not seem significant that an attacker can see how many bytes of their key were a match, but it can actually be **fatal** to security.
    The attacker can crack the first byte of the key by trying all 256 possibilities, and observing which one caused the comparison to take longer.
    Now, armed with the first byte, they can do the same with the second byte, and the third, and so on, until they have recovered the entire key.

With a lot of patience an adversary can recover secret information from an OpenSSL processes on the same host OS, an OpenSSL processes on a Virtual Machine on the same host OS, and even an OpenSSL on a separate host processes across a network.
It gets harder (read: more time consuming) to hack the farther away adversary, but they're all possible with sufficient patience, compute power, and Statistics\ :sup:`TM`.

Oh god fix it please
--------------------

Clam down!
We've come a long way since 2003 when this was proven to be a viable attack.
Most SSL libraries have fixed this vulnerability so you're fine as long as you updated in the past decade.

If you *haven't* updated in the past decade... burn that server.
Even the silicon atoms are compromised.
It's not even worth trying a fresh install.
The thing is just too far gone, start fresh.
Goodnight, sweet prince.

Constant-time Algorithms
------------------------

    How did the crypto libraries solve this problem?

        **Constant-time Algorithms**

    Oh fancy, tell me more.

Constant-time Algorithms are a way of implementing an algorithm in a way that always takes the same amount of time to compute regardless of the input.

Specifically, these perform in constant-time while processing *secret* information.
This distinction means processing a secret key *always* takes N cycles while checking that a configuration file is correctly formatted might take a M cycles or maybe M+5.

There are a lot of coding practices to be aware of in crypto which help us to avoid leaking information for Remote Timing Attacks.
Let's go over a few.

Limit conditionals on secrets
-----------------------------

    Avoid conditioning on secret information to avoid (among other things) CPU branch predictions.

Take this code for example:

.. code:: python

    if usually_true:
        do_usual_thing() # Path A
    else:
        do_weird_thing() # Path B

The CPU eventually will recognize that Path A is going to happen more than Path B so it will try to optimize for that path, making the "usual thing" faster.
This makes sense from a CPU designer standpoint [6]_, but it leaks information about which branch is being taken.
When the "unusual thing" happens the CPU has to backpedal before continuing, which takes a notable amount of time.
This backpedaling gives an adversary enough information to craft an attack the path they're on and extrapolate secret information based on that path-awareness.

*This type of attack (conditioning on private information) is explored in the Constnat Time Algorithm example and Remote Timing Attack demo near the end of the post.*

Division/Multiplication: tricky stuff
-------------------------------------

    Multiplication is not always constant-time.

Take this piece of code.

.. code:: text

    t1_a = current_time
    small_number_a * small_number_b
    t2_a = current_time

    t1_b = current_time
    big_number_a * big_number_b
    t2_b = current_time

    assert(t2_a - t1_a == t2_b - t1_b)

Believe it or not, even though the first and second blocks are just multiplying two numbers together they can take different amounts of time depending on your CPU and compiler.

This is triggered by some CPUs just not being equipped to handle large numbers, so they have to perform the large number multiplication in software.
Other CPUs optimize for small numbers since those get handled more frequently than large numbers.
These are pretty old hardware limitations, and the issue has mostly been resolved in newer 64-bit CPUs.
That said... you know... still something to lookout for.

The same goes for division.
Many CPUs don't have hardware support for division so the compiler needs to handle the operation in software.

TLDR: the same piece of code which is constant-time on one architecture (X86_64) might not be constant-time on another piece of hardware (x86_32 for example). [1]_

Compilers and undefined behavior
--------------------------------

    Watch out for compiler's "undefined behavior".

The C programming language, and *most* programming languages, have a formal specification of some kind.
This formal specification gives the programmer a good idea about what their code will do when they compile and run it.

For example if I wrote the following C:

.. code:: c

    int main() {
        return 10 + 20
    }

it *should* run and return ``30``, because the specification tells us that the ``+`` operator adds two numbers and ``return`` returns a given value from a function.
It also says that ``main`` returns a given integer as the exit status.

If I ran that code and it returned ``-1`` I'd be very confused; that breaks specification!
Compiler authors know this and follow the specification of the C language very carefully to make sure specified inputs produce specified outputs.

But what about behavior the specification *doesn't* mention?

Take for instance this:

.. code:: c

    int main() {
        fprintf("%d", 214748300 * 214745000);
    }

This might not be explicitly covered by the specification because it's pretty weird.
214748300 and 214745000 are close to the signed integer maximum, so when multiplied together what happens?
Will the program print an unsigned integer value of 4611686014132420609 or does it return a signed integer which has overflowed but is still signed?

This kind headache is called Undefined Behavior and it basically means the compiler, or rather the compiler's authors, *choose* which behavior they think is best because the language spec didn't say what should happen.

Another side effect hinted at is that while you can be confident what the end behavior of your code will be you can't predict how that behavior is achieved.
This didn't used to be an issue when C compilers were just 1:1 mapping your C loops and functions to sane assembly.
Fortunately compilers have gotten much better at producing fast and efficient executables.
**Unfortunately**, we aren't easily able to predict the runtime of our code because our compilers are liable to pour some black-magic voodoo on any and all binaries it produces.

These points are condensed really well by the BearSSL website:

    The C programming language is defined to run in an abstract machine under the "as if" rule, so the compiler is free to translate your code in any sequence of instructions that yield the expected result, with execution time not being part of the observable elements that must be preserved.

Even though we know that a function will always *work*, the C language (and compiler spec) doesn't care about *how* it gets done.

When you're trying to write crypto code this can feel like you're a parent telling your kid to clean their room.
They'll make it look clean, but they just shoved everything under the bed.
Technically they did what you wanted, the room looks cleaner, but they missed the point.
Something about building character in constant-time.

So what do we do?
We need to trick the compiler.

There are a handful of tricks to [5]_ to get the compiler to (a) avoid unknown behavior and (b) enforce a specific assembly output.
These are tricks include:

- Using a bit-wise operations instead of the equivalent mathematical operation.
- Mark important [secret] variables as volatile.
- Manually zero out important [secret filled] memory.
- Use multiple sources of entropy; as many as you can get your hands on.
- Read the output assembly and become a Jedi.

These are very high-level solutions to some of the problems, solutions I'm only going to hand-wavily describe, but honestly that's because I don't really grok the solutions and don't want to lead you astray.
Check out the end of this post for further reading by really smart people that get paid to do this stuff.

"Why can't I just..."
---------------------

Just *wait*?
If only.

So the first thought I had (and every other crypto novice has) is something like this:

.. code:: text

    do time sensitive operation
    sleep N seconds

This doesn't work because this just shifts the amount of time it takes to do an operation, literally *just* making your crypto take longer.
Then the *second* thought I and every other crypto novice has is something like:

.. code:: text

    record start time
    do time sensitive operation
    record end time
    sleep (expected time - elapsed time) seconds

This is *better* but you'll never *nail* the expected time.
It'll either be too long or too short
This means that either some amount of information is leaked, when ``expected time`` is too short, or the crypto is needlessly slow, which is just a silly compromise. [4]_

We *can* make it algorithmically secure without this ``sleep`` hack, so we *will*.

Practical Solutions to Timing Attacks
-------------------------------------

If you're implementing crypto, STOP.

If you're implementing crypto and you know what you're doing, your language of choice *probably* has a constant-time library which implements some primitives that you can take advantage of for simple tasks like comparisons.
Those can be found at the end of this post in Errata. [CTLibs]_

After looking into constant-time-ifying your code, do some reading and testing!
There have been a lot of developments in analyzing program constant-time-ness and much of this work is Open Source!
These are implemented using a lot of Statistics\ :sup:`TM`, code inspection, and even Valgrind. [10]_ [7]_ [11]_
If you're really concerned that *Your Crypto Library* isn't secure against Remote Timing Attacks, take one of those tools on a test drive and see what happens.
It's pretty likely that you'll find a *notable* timing difference based on different inputs and you'll probably need to make changes for your library to be secure against timing attacks.

It's for the greater good.
Because of your contributions the crypto community is even stronger.

A crazy idea: constant time language spec
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

My crazy idea, for my *very* hypothetical grad-school studies would be to implement a Constant-time *compiler* and/or *language*.
This would perform transformations to your code in an attempt to make it constant-time or warning you when your code isn't going to run in constant-time when it ought to.

Of course this would take a very long time, and honestly I haven't thought it through entirely, but I imagine something like this:

.. code:: text

    regular code

    ct {
        thing that needs to be constant-time.
    }

    regular code

Where you tell the compiler "This needs to be constant-time".
The compiler does it's best to convert loops and statements into constant-time and when it's done it tells you if it was able to convert your code into constant-time execution or not.

It's probably overkill; you don't usually just willy-nilly write constant-time code.
That said, just like C and Python help produce less error prone code than writing straight Assembly, so too might a constant-time language help produce code that hits less of the tricky pitfalls of implementing Constant-time algorithms.

Constant-time Algorithm example
-------------------------------

Take our code block from the beginning, the one that did the leaky comparison.
That takes different amounts of time when processing a given key against a known private key.
How would we write *that* in constant-time?

Something like this:

.. code:: python

    # Short circuit based on user input, does not leak private information
    if length(recieved_key) != PRIVATE_KEY_LENGTH:
        return KeyLengthError

    recvied_bits  = bits(recieved_key) # Taken to be constant-time
    expected_bits = bits(expected_key) # Taken to be constant-time

    ret = Thing

    for i in range(PRIVATE_KEY_LENGTH):
        if recieved_bits[i] != expected_bits[i]:
            ret = KeyLengthError # This sets the output, but the loop does not break

    return ret

This is similar to our original code but it does a few things differently:

#. We convert our keys to a variable which can be operated on bit-wise.
#. We manually compare each bit of the inputs. This is what the ``==`` operator does, but instead of returning when we get a difference we essentially set a switch. ``ret = Error`` from ``ret = Thing``.
#. Outside of the loop we return our response after processing all of our bits.

Behaviorally this is almost identical to our original code, but it does not return earlier or later depending on the user's input.

Yet another implementation avoids the direct comparison:

.. code:: python

    # Short circuit based on user input, does not leak private information
    if length(recieved_key) != PRIVATE_KEY_LENGTH:
        return KeyLengthError

    recvied_bits  = bits(recieved_key) # Taken to be constant-time
    expected_bits = bits(expected_key) # Taken to be constant-time

    matching_bits = 0

    for _ in range(PRIVATE_KEY_LENGTH):
        matching_bits += int(xor(recieved_bits[i], expected_bits[i]))

    if matching_bits == PRIVATE_KEY_LENGTH:
        return thing
    else:
        return KeyLengthError

Remote Timing Attack demo
-------------------------

    Examples are fine, but what about a demo!
    You said this was a real threat!

True, I did say that... so we'll do a small demonstration.

Below is a bit of Python code that checks a user's input against some hard-coded secret.

.. include:: ../.code/independent-crypto/secret.py
    :code: python

The thing that makes the above code particularly useful for our purposes is that it exaggerates the time it takes to evaluate the ``is_equal`` function.
Think of this as the 'backpedaling' the CPU does... turned up to 11.
Most important [for the author] we don't need to use Statistics\ :sup:`TM` to figure the secret, evaluating each input multiple times and collecting/processing that timing data, it already takes about one magnitude longer to evaluate a matching letter than it does to evaluate a non-matching letter.

Next we've got the attack code.

.. include:: ../.code/independent-crypto/attack.py
    :code: python

It's a bit convoluted in parts but if you stare at it for a while and read the enlightening comments you should see why this gets the right answer.

I encourage you to copy that code into two files, ``secret.py`` and ``attack.py`` and run it like so:

.. code:: text

    $ time python attack.py 
    character 0 is l
    character 1 is 3
    character 2 is 3
    character 3 is t
    Final guess is l33t

    real  0m35.176s
    user  0m1.300s
    sys 0m0.485s

    $ python secret.py l33t
    You got the secret!

You'll need a working Python installation and probably a shell of some kind.
I'm on CentOS Linux but any \*nix system will *probably* work.
With some fiddling you can probably get it to work on Windows ;-)

    This isn't a real threat!
    You exaggerated the problem!

Hush now.
It demonstrates the principles of the attack.
Besides, the post is over.
We've only got time for the conclusion and then you're off to bed.

Constant-time blogpost
----------------------

This topic has been a break from the theory/math-heavy term thus far.
Honestly I'm jazzed about it.

We learned that something as small as a comparison (a *comparison*!) can leak information to an adversary.
Your algorithm might be secure, but if you're not careful you can leak information in the most menial code.

This isn't a lost cause.
We don't need to throw this *security* thing out the window.
If we're aware of the gotchas we can craft code that solves these problems.
It's hard work but the peace of mind should make it worth it.

Learning about this seemingly obscure (*terrifying*) exploit in algorithmically secure code is just the kind of headache I enjoy in Computer Science.
Although I don't feel like I did Remote Timing Attacks justice, I could probably spend weeks on it, I had to call it quits.
I could keep working on this for *another 10 weeks*, but it's over.
Just walk away.

Annotated Bibliography
----------------------

BearSSL [1]_ [2]_
    BearSSL is a project which aims to make an architecture-independent constant-time implementation of various Crypto Libraries, largely mirroring compatibility with OpenSSL and related Open Source crypro libraries.
    Not only is the project interesting but a handful of blogposts and analysis are posted on the website covering topics like how to implement RSA in constant-time to the compatibility of various CPU models with assumptions about constant-time operations (e.g., multiplication).

Remote Timing Attacks are Practical [3]_
    This paper was very easy to read for an academic article and covered the creation of various timing attacks in practical conditions (e.g., hacking an RSA private key across a network).

Beginner focused blogs [4]_ [8]_
    These were two blogposts which laid out what timing attack, why they were viable, and how you can avoid them.
    Protip: never assume a library you're using is constant-time.

Adam Langley's blog [9]_ [11]_
    Adam Langley has great Intermediate-level blogposts about the Lucky13 attack and analyzing code for constant-time execution.
    These aren't for the weak of heart, but are much more accessible than a lot of academic articles on similar topics.

CryptoCoding.net Coding Rules [5]_
    This wiki outlines some common pitfalls when writing constant-time code and how to avoid it.
    It assumes you're writing C code, but many of the principles carry to more exotic languages.

Constant Time Testing Papers [7]_ [10]_
    These academic papers outline tools developed for studying how constant-time a program is and analyzes various programs with these tools.
    They're a great (surprisingly recent) survey of this topic.

Errata
------

.. [1] BearSSL: Constant Time Multiplication
       https://bearssl.org/ctmul.html

.. [2] BearSSL: Why Consant-Time Crypto?
       https://bearssl.org/constanttime.html

.. [3] Remote Timing Attacks are Pratical;
       David Brumley, Dan Boneh; http://crypto.stanford.edu/~dabo/pubs/papers/ssl-timing.pdf

.. [4] Chosen Plaintext: A beginner's guide to contant-time cryptography;
       https://www.chosenplaintext.ca/articles/beginners-guide-constant-time-cryptography.html

.. [5] Cryptography Coding Standards: Coding rules;
       https://cryptocoding.net/index.php/Coding_rules

.. [6] As Sonic the Hedgehog always says, Gotta Go Fast!

.. [7] Veryfying Constnat-Time Implementations, via Usenix;
       http://haslab.uminho.pt/jba/files/16usenix.pdf

.. [8] A Lesson In Timing Attacks (or, Don't use ``MessageDigest.isEquals``);
       Coda Hale; https://codahale.com/a-lesson-in-timing-attacks/

.. [9] Checking that functions are constant-time with Valgrind;
       Adam langley via ImperialViolet;
       https://www.imperialviolet.org/2010/04/01/ctgrind.html

.. [10] Dude, is my code constant-time?
        Oscar Reparaz, josep Balasch, Ingrid Vebauwhede;
        https://eprint.iacr.org/2016/1123.pdf

.. [11] Lucky Thirteen attack on TLS CBC
        Adam langley via ImperialViolet;
        https://www.imperialviolet.org/2013/02/04/luckythirteen.html

.. [CTLibs]
    - https://golang.org/pkg/crypto/subtle/
    - https://cryptography.io/en/latest/hazmat/primitives/constant-time/
    - https://github.com/isislovecruft/subtle
