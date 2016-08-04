---
title: blog
tags: [blog, blogroll]
date: 2016-03-16
type: page
filters: jinja2
---

{% for entry in env.globals.entrylist if 'blogpost' in entry.tags %}
* {{ entry.date.strftime("%Y-%m-%d") }} `{{ entry.title }}`_ {% for t in entry.tags %} {% if t != 'blogpost' %} [ {{ t }} ] {% endif %} {% endfor %}
{% endfor %}

{% for entry in env.globals.entrylist if 'blogpost' in entry.tags %}
.. _{{ entry.title }}: {{ entry.permalink }}
{% endfor %}
