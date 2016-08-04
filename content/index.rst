---
title: index
permalink: index.html
tags: [index]
type: page
date: 2016-03-16
filters: jinja2
---

|

.. class:: align-center

  **Recent Post:** `{{ env.globals.entrylist[0].title }}`_

My name is *Elijah C. Voigt*. I wear a few hats including:

- `Software Developer`_
- `OSU Open Source Lab Employee`_
- Oregon State University Student
- `OSU Linux Users Group President`_
- `Media Creator`_
- `Public Speaker`_

I'm not sure what you came here for. You might be looking for my `blog`_ which
has a mishmash of tech tool write-ups, travel blogposts, and other creative
endeavors.


If you would like to get in contact with me I have a page
for that too: `contact`_.

.. _{{ env.globals.entrylist[0].title }}: {{ env.globals.entrylist[0].permalink }}

.. _Software Developer: /about/#code
.. _OSU Open Source Lab Employee: /about/#osu-open-source-lab
.. _OSU Linux Users Group President:
.. _Media Creator: /about/#videos
.. _Public Speaker: /about/#slides
.. _blog: /blog/
.. _contact: /about/#contact
