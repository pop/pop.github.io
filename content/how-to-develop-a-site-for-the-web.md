+++
title = "How to develop a site for the web"

date = "2016-03-25"

description = "Testing your website with a web-server is important! Here's how to start testing `http://localhost:8000` and stop testing `file:///home/me/projects/website/site.html`."

taxonomies.tags = [
    "development"
]
+++

{% note() %}
This post does not deal with the basics of HTML, CSS, or Javascript, but
rather a simple and often overlooked part of the web-dev toolchain.

If you would like help with the basics of HTML, CSS, and Javascript I
suggest [Codecademy](https://codecademy.com/learn/make-a-website).
{% end %}

Web Development, especially the front-endy stuff, is a great way to
understand how computers *think* and can be a wonderful foray into
computer programming. Anybody that's made a website probably remembers
their first time:

1.  Open up a text editor.
2.  Type something like the following in:

```html
<h1>Hello world!</h1>
<p>This is so cool!</h1>
```

3.  `Save As index.html`.
4.  Right click and open the file in your browser.
5.  Marvel at what you've created and hack away at it all night long.

And there you go! You've got a website made and ready to roll. It's not
on a server, and you can't tell your friends to go to it from *their*
computer --but those are just technicalities. You can still celebrate
doing a thing *like a boss*.

Unfortunately, when you test your website by viewing local files you're
missing out on a lot of advantages and quirks that you get when you host
a website on a *real server* using something like [Apache or
Nginx](https://en.wikipedia.org/wiki/Web_server).

For instance, when you run check your website by clicking through
`file///home/username/project/files.html` all of your hyperlinks that
should point to `http://mywebsite.ext/somepage/` will take you to
`file:////somepage` when they should take you to
`file:///home/username/projects/myawesomesite/somepage/index.html`.

## Run A Development Server (it's easy!)

### The Short Answer

**Use Python!**

```txt
$ python2 -m SimpleHTTPServer
Serving HTTP on 0.0.0.0 port 8000 ...
```

### The Longer Answer

You can run your own local webserver to serve files *locally*.
This gets you all of the developmental advantages of running a webserver without having to rent or run a server in the cloud.
The best part is that it's very easy to run a development web server.

Here's how:

1.  [Install Python](https://python.org/downloads/)
2.  Navigate to `your-awesome-website` directory/folder.
3.  Run `python -m SimpleHTTPServer` if you installed `python2` or run `python -m http.server` if you installed `python3`.

{% note() %}
These instructions assume you are doing things in a Unix environment (OSX or Linux) thought the commandline.
If you are using Windows you should check out this [stack overflow post](http://stackoverflow.com/questions/17351016/set-up-python-simplehttpserver-on-windows) for Windows-specific help.
{% end %}

Then in your web browser go to the address `http://localhost:8000` and
bam! **Your** Website.

![High Five.gif](/images/gifs/high-five.gif)
