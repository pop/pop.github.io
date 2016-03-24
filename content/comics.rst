---
title: comics
tags: [comics]
date: 2016-03-16
type: page
filters: jinja2
---

{% for entry in env.globals.entrylist if 'comic' in entry.tags %}
* {{ entry.date.strftime("%Y-%m-%d") }} `{{ entry.title }}`_
{% endfor %}

{% for entry in env.globals.entrylist if 'comic' in entry.tags %}
.. _{{ entry.title }}:  {{ entry.permalink }}
{% endfor %}
