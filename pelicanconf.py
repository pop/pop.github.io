#!/usr/bin/env python
# -*- coding: utf-8 -*- #
from __future__ import unicode_literals

AUTHOR = 'Elijah Voigt'
SITENAME = 'elijahcaine.me'
SITEURL = 'elijahcaine.me'

PATH = 'content'

TIMEZONE = 'America/Los_Angeles'

DEFAULT_LANG = 'en'

# Feed generation is usually not desired when developing
FEED_ATOM = "atom.xml"
FEED_RSS = "rss.xml"

DEFAULT_PAGINATION = 10

# Uncomment following line if you want document-relative URLs when developing
RELATIVE_URLS = True

ARTICLE_URL = '{slug}'
ARTICLE_SAVE_AS = '{slug}/index.html'
DRAFT_URL = "drafts/{slug}"
DRAFT_SAVE_AS = 'drafts/{slug}/index.html'
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

THEME = "theme/porterjamesj-fork"

SOCIAL = (('github', 'https://github.com/elijahcaine'), ('mailbox', 'mailto:elijahcainemv@gmail.com'))

AUTHORS_BIO = {
  "elijah": {
    "name": "Elijah C. Voigt",
    "cover": "/assets/images/hobbes.jpg",
    "image": "/assets/images/calvin-and-hobbes.jpg",
    "website": "http://elijahcaine.me",
    "location": "Silicon Forest",
    "bio": (
        "Elijah Caine is an Oregon State University alumni who has interned at the Open Source Lab, CoreOS, and Nordstrom. "
        "He has worked as a full time employee at Nordstrom as a 'Unix DevOps Engineer' and now works at CloudBolt software as a Django developer. "
        "In his free time he creates starts projects he can't seem to finish."
    ),
  }
}

from datetime import datetime
NOW = datetime.now()

LICENSE_NAME = "Creative Commons Attribution 4.0 International"
LICENSE_URL = "http://creativecommons.org/licenses/by/4.0/"
LICENSE_IMG = '<img alt="Creative Commons License" style="border-width:0" src="/assets/images/CCA4IL.png" />'
MENUITEMS = [("(🏠 elijahcaine.me)", "/"),("(🐙 github 🐱)", "https://github.com/pop/"),("(👔 resume)", "/resume.pdf")]
FOOTER_WHIMSY = (
    "This website made with locally sourced bits, "
    "built with Python 🐍 via Pelican 🐦, "
    "developed on Fedora 👒 GNU/Linux 🐃🐧, "
    "<a href='https://github.com/pop/elijahcaine.me'>hosted</a> and deployed on GitHub 🐙🐱, "
    "and served from clouds ⛅."
)
