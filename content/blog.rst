---
title: blog
date: 2016-03-16
type: page
filters: jinja2
---

Blog
====

{% for entry in env.globals.entrylist %}
{% if "archive" not in entry.tags %}

.. I know it's gross.

.. raw:: html

    <div class="blog-roll"
          title="[ {{ entry.date.strftime("%Y-%m-%d") }} ] | {% for t in entry.tags %} [ {{ t }} ] {% endfor %}">

- [ {{ entry.date.strftime("%Y-%m-%d") }} ] `{{ entry.title }}`_

.. raw:: html

    </div>

{% endif %}
{% endfor %}


{% for entry in env.globals.entrylist %}
{% if "archive" not in entry.tags %}
.. _{{ entry.title }}: {{ entry.permalink }}
{% endif %}
{% endfor %}
