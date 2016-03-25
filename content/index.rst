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

  **Newest Blogpost**: `{{ env.globals.entrylist[0].title }}`_

|

.. figure:: https://media.giphy.com/media/IBMavwmu4KEEw/giphy.gif
    :target: https://giphy.com/gifs/cheezburger-hello-waving-IBMavwmu4KEEw
    :align: center
    :alt: Whale Hello There!

|

Thank's for vising my website. It's got the Bootstraps and the JQuerys, so it's
pretty on all the things.

If you find any problems or have feedback to give, email me at
elijahcainemv@gmail.com.

.. _the blog: /blog/
.. _{{ env.globals.entrylist[0].title }}: {{ env.globals.entrylist[0].permalink }}
