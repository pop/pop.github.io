---
title: How to Possibly Kinda Sorta Write a Travelblog
date: 2015-09-29
tags: [blogpost, travel, write-the-docs]
type: entry
draft: false
---

How To Possibly Kinda Sorta Write a Travelblog
==============================================

    **First thing's First: I went on a trip and wrote a travelblog.**

    **For a link to that travelblog itself check out** `elijahcaine.me/eurotrip-2015`_

    **This post is _about_ that travelblog.**


I went on a whirlwind two week trip to Europe; first to Amsterdam ('because
Amsterdam' seems like a reasonable justification for doing this according to
most people) and second to Prague to speak at the `European Write the Docs
conference`_.

I've traveled before and I wanted to avoid the inevitable that tends to happen:

#. I go on a trip.
#. Have a wonderful time and make lots of memories.
#. Get back from trip.
#. Get asked "Tell me about your trip!"
#. Forget everything that happened on the trip and sum it up with 'It was
   great.'

To combat this seemingly inevitable outcome I decided to write a blog about
**this** trip as the trip was happening. Similar to how some might make daily
Facebook posts or Tweets, I could use this as a way to not only spend time each
day processing what happened but I could also refer friends and family to the
blog's url so they could follow along vicariously if they chose. I wanted to
make a website where I updated the world on my whereabouts and shenanigans.

And so I did.

The Technical Parts
-------------------

I'm an engineer. Like many engineers I like to tinker and build things.

A while ago I built a tool called `PageUp`_; it takes a `reStructuredText`_
formatted `markup file`_, an `html template`_, as well as some other files that
make the web go 'round and look half-way decent.

Using that tool (PageUp) I created a blog that I could update from my phone
using `a Github Gist`_ and a slick `Git Source Control`_ program for my phone.
I could write a reStructuredText formatted blog post, sync it to the web, have
my personal server pull the updated version of the `git repository`_, and
automatically rebuild the website.

The Writing Parts
-----------------

Updates were surprisingly painless-- and I'm not just saying that to sell
PageUp, I'm honestly just impressed that I made something which actually made
my life easier. Feels good.

I wasn't as consistent with writing posts as I had envisioned. I wrote posts
the day after the events occurred, which wasn't too bad, until I went to Write
the Docs got completely torn away from my rigorous 'Drink Coffee, Write
Blogpost, Adventure' regimen; I kept getting distracted by meeting new people
and socializing. Ugh, the worst.

Thoughts and Conclusions
------------------------

All in all I suspect I'll continue doing liveblogs when I go to events, it's a
great way to share the experience with others and augment one's memory with a
written record of the time you spent.

Plus it's just fun an nerdy to write scripts and programs to make this whole
experience as user friendly as possible. If I poured even more time into this
it could even be a startup...

.. _elijahcaine.me/eurotrip-2015: http://elijahcaine.me/eurotrip-2015
.. _European Write the Docs conference: http://www.writethedocs.org/conf/eu/2015/schedule/
.. _PageUp: https://github.com/elijahcaine/pageup#pageup-init-pageup-build
.. _reStructuredText: https://en.wikipedia.org/wiki/ReStructuredText
.. _markup file: https://en.wikipedia.org/wiki/Markup_language
.. _html template: https://en.wikipedia.org/wiki/Web_template_system
.. _a Github Gist: https://gist.github.com/ElijahCaine/352cb120743af2dde7c8
.. _Git Source Control: http://www.git-scm.com/
.. _git repository: https://gist.github.com/ElijahCaine/352cb120743af2dde7c8

.. _PageUp on Github: https://github.com/elijahcaine/pageup
