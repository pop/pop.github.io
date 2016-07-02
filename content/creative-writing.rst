---
title: creative writing
tags: []
date: 2016-07-01
type: page
filters: jinja2
---

Below are some pieces of creative writing I have chosen to publish. Enjoy :)

{% for entry in env.globals.entrylist if 'creative-writing' in entry.tags %}
* {{ entry.date.strftime("%Y-%m-%d") }} `{{ entry.title }}`_
{% endfor %}

{% for entry in env.globals.entrylist if 'creative-writing' in entry.tags %}
.. _{{ entry.title }}:  {{ entry.permalink }}
{% endfor %}
