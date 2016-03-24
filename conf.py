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
    # main pages
    '/:slug/': {'filters': ['h2', 'nohyphenate'], 'view': 'page', 'template': 'page.html', 'if': lambda e: 'page' == e.type},

    # liveblog(s)
    '/liveblog/:slug/': {'filters': ['h2', 'nohyphenate'], 'view': 'page', 'template': 'page.html', 'if': lambda e: 'liveblog-meta' in e.tags},
    '/liveblog/europe-2015/:slug/': {'views': ['entry', 'draft'], 'template': 'post.html', 'if': lambda e: 'liveblog' in e.tags},

    # blog posts
    '/blog/:slug/': {'views': ['entry', 'draft'], 'template': 'post.html', 'if': lambda e: 'blog' in e.tags},

    '/atom/': {'filters': ['h2', 'nohyphenate'], 'view': 'atom'},
    '/rss/': {'filters': ['h2', 'nohyphenate'], 'view': 'rss'},

    '/sitemap.xml': {'view': 'sitemap'},

    #'/tag/:name/': {'filters': 'summarize', 'view':'tag',
    #                'pagination': '/tag/:name/:num/'},
}

THEME = 'theme'
ENGINE = 'acrylamid.templates.jinja2.Environment'
DATE_FORMAT = '%Y-%m-%d'
