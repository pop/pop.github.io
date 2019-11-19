# elijahcaine.me

This is the git repo for my blog.
Thanks for checkit it out!

## theme

First off, big thanks to [porterjamesj] on GitHub for letting me fork his theme for this blog.
I like the theme a lot.

## getting started

To run the website locally, you should probably first install [Poetry].

Once you've done that, run the following to get your environment in order:

```sh
$ poetry install
Creating virtualenv elijahcaine.me-kmgybYzN-py3.7 in /home/myuser/.cache/pypoetry/virtualenvs
Installing dependencies from lock file


Package operations: 11 installs, 0 updates, 0 removals

  - Installing markupsafe (1.0)
... snip ...
  - Installing pelican (4.2.0)
```

Great! Now just run this to get the dev server running:

```sh
poetry run devserver
```

This will auto-reload the server every time you update a content file.
How cool!

## contributing

If you'd like to contribute by suggesting edits or even adding an article to the blog, make an issue or pull request.

Thanks!

[Poetry]: https://poetry.eustace.io
[porterjamesj]: https://github.com/porterjamesj/
