---
title: index
permalink: index.html
tags: [index]
type: page
date: 2016-08-19
filters: jinja2
---

Homepage
========

My name is *Elijah C. Voigt*. I wear a few hats including `Software
Developer`_, `OSU Open Source Lab Employee`_, Oregon State University Student,
`OSU Linux Users Group President`_, `Media Creator`_, `Public Speaker`_, `Book
Reader`_, and `Writer`_.

I'm not sure what you came here for. You might be looking for my `blog`_ which
has a mishmash of tech tool write-ups, travel blogposts, and other creative
endeavors. The most recent post is just down the page.

If you would like to get in contact with me I have a page for that too
`Contact Me`_.

----

.. class:: align-center

  [ `Recent Post`_ ]

{{ env.globals.entrylist[0].source }}


.. class:: align-center

    [ `More Posts`_ ]

.. _Recent Post: {{ env.globals.entrylist[0].permalink }}

.. _Software Developer: /about/#code
.. _OSU Open Source Lab Employee: /about/#osu-open-source-lab
.. _OSU Linux Users Group President: http://lug.oregonstate.edu/contact/
.. _Writer: /blog/
.. _Media Creator: /about/#videos
.. _Public Speaker: /about/#slides
.. _Book Reader: /about/#reading
.. _blog: /blog/
.. _Contact Me: /about/#contact
.. _More Posts: /blog/
