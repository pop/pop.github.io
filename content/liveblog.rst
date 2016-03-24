---
title: liveblog
date: 2016-03-17
tags: [liveblog]
type: page
filters: jinja2
---

When I do something exciting I try to document the experience in *Real Time™*
to help reflect and articulate my thoughts and feelings about the experience.
This page is a compilation of those *liveblogs* as I have chosen to call them.

Current Adventure
=================

.. class:: align-center

    **HTTP Error 204: No Content**

Past Adventure
==============

{% for page in env.globals.pages if 'liveblog-adventure' in page.tags %}
* `{{ page.title }}`_
{% endfor %}

{% for page in env.globals.pages if 'liveblog-adventure' in page.tags %}
.. _{{ page.title }}: {{ page.permalink }}
{% endfor %}
