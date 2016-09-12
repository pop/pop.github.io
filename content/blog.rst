---
title: blog
date: 2016-03-16
type: page
filters: jinja2
---

Blog
====

{% for entry in env.globals.entrylist %}

.. I know it's gross.

.. raw:: html

    <div class="blog-roll"
          title="[ {{ entry.date.strftime("%Y-%m-%d") }} ] | {% for t in entry.tags %} [ {{ t }} ] {% endfor %}">

- `{{ entry.title }}`_

.. raw:: html

    </div>

{% endfor %}


{% for entry in env.globals.entrylist %}
.. _{{ entry.title }}: {{ entry.permalink }}
{% endfor %}
