---
title: Europe 2015
date: 2015-09-19
type: page
tags: [liveblog-adventure]
filters: jinja2
---

`My`_ little liveblog / travelblog thing, covering everything 
leading up to and including my trip to Amsterdam and Prague to 
speak at `Write the Docs - Europe`_.

Please excuse the spelling errors, English is only my first 
language.

Oldest posts are at the bottom, newer posts are at the top.

\* Well-After-The-Fact = It was originally a liveblog, then a live(ish) blog,
then I took two weeks off to write the last three days worth of posts --
basically I'm just glad I was able to stick with it to the end, even if it was
a bit delayed getting finished.

.. note::

    Posts for this liveblog have ceased. Keep an eye out for my next trip when
    I will try my hand at live-blogging again :)

.. _My: http://elijahcaine.me
.. _Write the Docs - Europe: http://www.writethedocs.org/conf/eu/2015/speakers/

Posts from this Adventure:
--------------------------

{% for entry in env.globals.entrylist if 'europe-2015' in entry.tags %}
* {{ entry.date.strftime("%Y-%m-%d") }} `{{ entry.title }}`_
{% endfor %}

{% for entry in env.globals.entrylist if 'europe-2015' in entry.tags %}
.. _{{ entry.title }}: {{ entry.permalink }}
{% endfor %}
