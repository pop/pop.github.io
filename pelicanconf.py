#!/usr/bin/env python
# -*- coding: utf-8 -*- #
from __future__ import unicode_literals

AUTHOR = 'Elijah Caine'
SITENAME = 'elijahcaine.me'
SITEURL = 'elijahcaine.me'

PATH = 'content'

TIMEZONE = 'America/Los_Angeles'

DEFAULT_LANG = 'en'

# Feed generation is usually not desired when developing
FEED_ALL_ATOM = "atom.xml"
FEED_ALL_RSS = "rss.xml"
CATEGORY_FEED_ATOM = None
TRANSLATION_FEED_ATOM = None
AUTHOR_FEED_ATOM = None
AUTHOR_FEED_RSS = None

# Blogroll
LINKS = (('Pelican', 'http://getpelican.com/'),
         ('Python.org', 'http://python.org/'),
         ('Jinja2', 'http://jinja.pocoo.org/'),
         ('You can modify those links in your config file', '#'),)

# Social widget
# SOCIAL = (('You can add links in your config file', '#'),
#           ('Another social link', '#'),)

DEFAULT_PAGINATION = 10

# Uncomment following line if you want document-relative URLs when developing
RELATIVE_URLS = True

ARTICLE_URL = '{category}/{slug}'
ARTICLE_SAVE_AS = '{category}/{slug}/index.html'
DRAFT_URL = "drafts/{category}/{slug}"
DRAFT_SAVE_AS = 'drafts/{category}/{slug}/index.html'
DEFAULT_CATEGORY = 'misc'
DISPLAY_PAGE_ON_MENU = True
STATIC_PATHS = ['assets']
EXTRA_PATH_METADATA = {
  'assets/CNAME': {'path': 'CNAME'},
  'assets/resume.pdf': {'path': 'resume.pdf'},
  'assets/hobbes.ico': {'path': 'favicon.ico'},
  'assets/garbled-circuits-game.html.out': {'path': 'garbled-circuits-game.html'},
}

# Author
AUTHOR_URL = 'author/{slug}'
AUTHOR_SAVE_AS = 'author/{slug}/index.html'
AUTHORS_SAVE_AS = 'authors.html'

THEME = "theme/mnmlist-fork"

SOCIAL = (('github', 'https://github.com/elijahcaine'), ('mailbox', 'mailto:elijahcainemv@gmail.com'))

COLOR_SCHEME_CSS = 'monokai.css'

AUTHORS_BIO = {
  "elijah": {
    "name": "Elijah Caine",
    "cover": "/assets/images/hobbes.jpg",
    "image": "/assets/images/calvin-and-hobbes.jpg",
    "website": "http://elijahcaine.me",
    "location": "Silicon Forest",
    "bio": "Elijah Caine is an Orego State University alumni who has interned at the Open Source Lab, CoreOS, and Nordstrom. In his free time he creates problems he can regret not solving."
  }
}

GOOGLE_ANALYTICS = "UA-48615202-3"
