---
title: index
permalink: index.html
type: page
date: 2016-08-19
filters: jinja2
---

Blog
====

{% for entry in env.globals.entrylist %}

.. I know it's gross.

.. raw:: html

    <div class="blog-roll"
          title="{% for t in entry.tags %} [ {{ t }} ] {% endfor %}">

- [ {{ entry.date.strftime("%Y-%m-%d") }} ] `{{ entry.title }}`_

.. raw:: html

    </div>

{% endfor %}


{% for entry in env.globals.entrylist %}
.. _{{ entry.title }}: {{ entry.permalink }}
{% endfor %}
