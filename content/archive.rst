---
title: archive
date: 2016-12-13
type: page
filters: jinja2
---

Archive
=======

{% for entry in env.globals.entrylist %}

.. I know it's gross.

.. raw:: html

    <div class="blog-roll"
          title="[ {{ entry.date.strftime("%Y-%m-%d") }} ] | {% for t in entry.tags %} [ {{ t }} ] {% endfor %}">

- [ {{ entry.date.strftime("%Y-%m-%d") }} ] `{{ entry.title }}`_

.. raw:: html

    </div>

{% endfor %}

{% for entry in env.globals.entrylist %}
.. _{{ entry.title }}: {{ entry.permalink }}
{% endfor %}
