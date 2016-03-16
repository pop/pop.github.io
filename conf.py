# -*- encoding: utf-8 -*-
# This is your configuration file.  Please write valid python!
# See http://posativ.org/acrylamid/conf.py.html

SITENAME = 'A descriptive blog title'
WWW_ROOT = 'http://example.com/'

AUTHOR = 'Anonymous'
EMAIL = 'mail@example.com'

FILTERS = ['markdown+codehilite(css_class=highlight)', 'hyphenate', 'h1']
VIEWS = {
    '/:slug/': {'view': 'page', 'template': 'page.html'},
    '/blog/:slug/': {'views': ['entry', 'draft'], 'template': 'post.html'},

    '/atom/': {'filters': ['h2', 'nohyphenate'], 'view': 'atom'},
    '/rss/': {'filters': ['h2', 'nohyphenate'], 'view': 'rss'},

    '/sitemap.xml': {'view': 'sitemap'},

    #'/tag/:name/': {'filters': 'summarize', 'view':'tag',
    #                'pagination': '/tag/:name/:num/'},
}

THEME = 'theme'
ENGINE = 'acrylamid.templates.jinja2.Environment'
DATE_FORMAT = '%d.%m.%Y, %H:%M'
