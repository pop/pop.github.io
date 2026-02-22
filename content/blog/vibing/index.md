+++
title = "Vibing"
date = "2026-02-21"
description = "I start hanging out with Claude and I don't hate it yet"
draft = true
+++

> You can check out this blog's editor at https://editor.elijah.run.
> It supports anonomous browsing and is under very active development.

If for some reason you follow the history of this repo you might have noted a lot of commits co-authored by Claude Code recently.
That is because I am... vibe coding.

# What am I vibing up over here?

For a long while I've wanted to be able to blog on the go.
Finish a book, whip out my phone, write a post, and publish it all without opening my laptop.
Alas, most blogs that have a web-editor like that are dynamic sites with databases and server-side code but this is a static site with templates and markdown that get built by a CI job.

But a few months ago I started to take the idea seriously: how _would_ I build a web editor for this blog?
* I toyed around with the idea of emailing posts to myself, but then I couldn't easily edit.
* I could migrate to a dynamic site, but I didn't want to deal with hosting costs.
* Maybe I could git clone my blog and push commits, but there aren't any good git clients on Android(?!)

I finally landed on creating a one-page web app that used the Github API to create branches, update content, check CI for mergability, and merge when ready.
The only problem: I didn't want to build it.

# Why are you vibing?

I have a wonderful full life with a job, two little kids, and plenty of hobby satisfaction building games with Bevy.
The last thing I need is to spend months of my fleeting free time learning a front-end framework, the github API, how Oauth works, how Cloudflare hosting works (for their free tier), just to probably burn out on the project before it's even usable.

Enter: Claude Code.
I've been using Claude Code since it was first announced like ~30 AI-years ago and immediately fell in love.
That said, this was exclusively in the context of work.
I would buy credits, expense them, and just use it for troubleshooting bugs that I got reported.
I did use it to do a coding exercise for a job interview which thankfully was the point of the exercise: give you an impossible task and expect you to use AI without saying it's allowed or not allowed; spoiler I got the job.

