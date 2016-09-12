---
title: blog
date: 2016-03-16
type: page
filters: jinja2
---

Blog
====

{% for entry in env.globals.entrylist if 'blogpost' in entry.tags %}
* `{{ entry.title }}`_ {% for t in entry.tags %} [ {{ t }} ] {% endfor %}
{% endfor %}

{% for entry in env.globals.entrylist if 'blogpost' in entry.tags %}
.. _{{ entry.title }}: {{ entry.permalink }}
{% endfor %}
