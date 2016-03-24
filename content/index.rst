---
title: index
permalink: index.html
tags: [index]
type: page
date: 2016-03-16
filters: jinja2
---

.. figure:: https://media.giphy.com/media/IBMavwmu4KEEw/giphy.gif
    :target: https://giphy.com/gifs/cheezburger-hello-waving-IBMavwmu4KEEw
    :align: center
    :alt: Whale Hello There!

Thank's for vising my website. It's got the Bootstraps and the JQuerys, so it's
pretty on all the things.

It's still a work in progress, but check out `the blog`_, that's mostly
finished.

{% for entry in env.globals.entrylist %}
{{ entry.title }}
{{ entry.tags }}
{% endfor %}

.. _the blog: /blog/
