+++
title = "Independent Crypto Course syllabus"

date = "2017-07-10"

description = "Independent Crypto landing page."

taxonomies.tags = [
    "independent crypto"
]
+++

{% note() %}
This is a part of a series of blog posts I wrote for an Independent Study on cryptography at Oregon State University.
To read all of the posts, check out the 'Independent Crypto' tag.
{% end() %}

{% warning() %}
This syllabus was written by an Oregon State University undergraduate student and not by an Oregon State University staff member.
This should explain any irregularities in the structure and substance of the document.
{% end %}

The purpose of this course ("Independent Crypto") is to give students an opportunity to dive deeper into interesting topics of Cryptography.

By the end of this course you should *grok* the following topics:

- Elliptic curve cryptography
- Memory hard functions
- Garbled circuits
- An topic of your choosing (get creative!)

Each of these should take about 40 hours of work to complete.
An overview of what that means is outlined below.

{% note() %}
This course is designed to be a 4 credit hour independent study.

As is standard Oregon State University policy, this corresponds with 160
hours of work over a 10 week period.
Plan accordingly.
{% end %}

------------------------------------------------------------------------

Each topic ought to take about 40 hours of work to complete over the course of a 10 week term.
If you schedule your time well this will be a piece of cake at just 16 hours per week.
That's 2.28 hours per day, 3.2 hours per week-day, or 16 hours the day before your check-in is due!

The basic structure is as follows:

1.  Research a topic.
    Read papers, watch informative videos, ask questions and learn as much as you can about a given topic.
2.  Maintain an *annotated bibliography*.
    This should include materials found while studying a given topic, a summary of each of the materials, and a final summary of the topic as a whole.
3.  Meet weekly with the mentoring professor.
4.  Repeat.
5.  ???
6.  Profit.

The end goal, in addition to learning about a breadth of topics in modern security, is to produce an annotated bibliography.
This will exercise the student's ability to read and process academic topics, journals, and videos.

Of course if you are particularly passionate about a topic you are encouraged to go further: implementing things of interest, investigating new questions, and generally 'digging deeper' as you gain interest in different topics.

The following topics do not *need* to be completed in order, however doing so will result in an optimal 'difficulty curve' as the kids say.
The kids do still say that right?

{% note() %}
Included are a few resources grabbed in a quick internet search.
These are meant to be starting places for each topic, generating questions and providing external resources.
You will need to find additional resources for each topic.
{% end %}

## Elliptic Curve Cryptography

Weeks 0-2 will be dedicated to Elliptic Curve Cryptography.

### Kickoff Resources

- A (relatively easy to understand) primer on Elliptic Curve Cryptography: <https://arstechnica.com/security/2013/10/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/>
- Elliptic Curve Cryptography in Practice: <https://eprint.iacr.org/2013/734.pdf>
- Elliptic Curve Cryptography, a gentle introduction: <http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/>

### Kickoff Questions

- What are elliptic curves?
- How do elliptic curves relate to cryptography?
- How are Elliptic Curve Cryptography functions different from similar ones like RSA?

## Memory-hard functions

Weeks 3 and 4 of the course should be dedicated to the topic of Memory Hard Functions.

### Kickoff Resources

- Memory-hard functions and tradeoff cryptanalysis with applications to password hashing, cryptocurrencies, and white-box cryptography: <https://www.cryptolux.org/images/d/d1/Tradeoff-slides.pdf>
- Strict Memory Hard Hashing Functions: <http://www.hashcash.org/papers/memohash.pdf>

### Kickoff Questions

- What are Memory-hard functions?
- What purposes are Memory-hard functions used for?
- What are some examples of Memory-hard functions and how do they work?

## Garbled circuits

Weeks 5-7 should be dedicated to garbled circuits.

### Kickoff Resources

- Garbled Circuits: <https://youtu.be/TxCu1L_tzlU>
- Foundations of Garbled Circuits: <https://eprint.iacr.org/2012/265.pdf>
- Faster Secure Two-Party Computation Using Garbled Circuits: <https://www.usenix.org/legacy/event/sec11/tech/full_papers/Huang.pdf>
- SFE: Yao’s Garbled Circuit: <https://courses.engr.illinois.edu/cs598man/fa2009/slides/ac-f09-lect16-yao.pdf> (bonus points if you find the talk for these slides).
- Garbled Circuts, Cryptowiki, <http://cryptowiki.net/index.php?title=Garbled_circuits> (probably don't cite this one in a paper)
- Amortizing Garbled Circuits: <https://eprint.iacr.org/2015/081.pdf>

## Independent study

In the last seven or so weeks you've learned a lot.
You've read papers, watch informative lectures, and had insightful conversations with peers and mentors.
Many of these probably sparked your attention in a particular topic.
Use these last few weeks to investigate one of those sparks that you've been itching to learn more about.

If you truly feel uninspired you can use this time to learn about Private Set Intersection.

### Kickoff Resources for Private Set Intersection

- BIU Winter School on Cryptography on Youtube.
- CSCI E-127, Introduction to Cryptography <http://cm.dce.harvard.edu/2014/01/14301/publicationListing.shtml>

### Generic Kickoff Questions

- What topic are you investigating?
- How does this relate to Cryptography?
- How would you explain this topic to your friends or parents?
- Why is this topic important?
- What interests you about this topic?

## Additional resources

OSU Professor [Mike Rosulek](http://web.engr.oregonstate.edu/~rosulekm/) volunteered the following additional resources:

- Scrypt is maximally memory-hard, <http://www.cs.bu.edu/fac/reyzin/papers/BostonCryptoDayTalk-Leo.pptx>
- Practical Graphs for Optimal Side-Channel Resistant Memory-Hard Functions, <https://eprint.iacr.org/2017/443.pdf>
- Efficiently Computing Data Independent Memory Hard Functions, <https://youtu.be/ujpvPtn_N5Y>
- Memory hard Functions and Password Hashing, <https://youtu.be/9yX4v89m5oo>
- Towards a Theory of Data-Independent Memory Hard Functions, <https://youtu.be/YtfVLzUkwME>
- Depth-Robust Graphs and Their Cumulative Memory Complexity, <https://eprint.iacr.org/2016/875.pdf>
- Practical Garbled Circuit Optimizations, <http://web.engr.oregonstate.edu/~rosulekm/pubs/gc-survey-talk.pdf>
- Pratical Garbled Circuit Optimizations, <https://youtu.be/FTxh908u9y8>
- Cache-timing attacks on AES, <http://cr.yp.to/antiforgery/cachetiming-20050414.pdf>
- Lucky Thirteen attack on TLS CBC, <https://www.imperialviolet.org/2013/02/04/luckythirteen.html>
