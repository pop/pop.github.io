# -*- encoding: utf-8 -*-
# This is your configuration file.  Please write valid python!
# See http://posativ.org/acrylamid/conf.py.html

SITENAME = 'Elijah\'s Website'
WWW_ROOT = 'http://elijahcaine.me/'

AUTHOR = 'Elijah Caine'
EMAIL = 'elijahcainemv@gmail.com'

STATIC = 'static'
CONTENT_IGNORE = ['.*.swp', '.*.swo', '*.scss~', '.sass-cache/', '*.map']

FILTERS = ['rst+codehilite(css_class=highlight)', 'hyphenate', 'h1']
VIEWS = {
    '/:slug/': {'filters': ['h2', 'nohyphenate'], 'view': 'page', 'template': 'page.html', 'if': lambda e: not 'liveblog' in e.tags},
    '/liveblog/:slug/': {'filters': ['h2', 'nohyphenate'], 'view': 'page', 'template': 'page.html', 'if': lambda e: 'liveblog' in e.tags},
    '/blog/:slug/': {'views': ['entry', 'draft'], 'template': 'post.html'},

    '/atom/': {'filters': ['h2', 'nohyphenate'], 'view': 'atom'},
    '/rss/': {'filters': ['h2', 'nohyphenate'], 'view': 'rss'},

    '/sitemap.xml': {'view': 'sitemap'},

    #'/tag/:name/': {'filters': 'summarize', 'view':'tag',
    #                'pagination': '/tag/:name/:num/'},
}

THEME = 'theme'
ENGINE = 'acrylamid.templates.jinja2.Environment'
DATE_FORMAT = '%Y-%m-%d'
