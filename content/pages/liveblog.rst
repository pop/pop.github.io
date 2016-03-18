---
title: liveblogs
date: 2015-09-29
tags: liveblog page
type: page
filters: jinja2
---


{% for page in env.globals.pages %}
{% if env.permalink != page.permalink %}
`{{ page.title }}`_
{% endif %}
{% endfor %}

{% for page in env.globals.pages %}
{% if env.permalink != page.permalink %}
.. _{{ page.title }}: {{ page.permalink }}
{% endif %}
{% endfor %}
