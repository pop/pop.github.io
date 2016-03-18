---
title: blog
tags: blog blogroll posts
date: 2016-03-16
type: page
filters: jinja2
---

{% for entry in env.globals.entrylist %}
* {{ entry.date.strftime("%Y-%m-%d") }} `{{ entry.title }}`_
{% endfor %}

{% for entry in env.globals.entrylist %}
.. _{{ entry.title }}:  {{ entry.permalink }}
{% endfor %}
