+++
title = "Vibing"
date = "2026-02-21"
description = "I start hanging out with Claude and I don't hate it yet"
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

So I was familiar with Claude Code, but didn't actively keep up with it or any other LLM news, until I (and most of this corner of the internet) read [Something Big is Happening](https://shumer.dev/something-big-is-happening) which as intended scared the bejesus out of me.
Upon reflection most of that post is too rich for my blood, but the takeways that stuck with me have been:
1. The tools are better than you remember. Try the latest models.
2. Focus on being a builder and a learner.

The first was a wakeup call, the second is how I am choosing to think of these tools and will hopefully train my kids how to use them.
LLMs are a great tool for polyglots, down to tutor you on any topic at any time of day and for as long as you like -- but that's a post for another day.

Around this time I also read about [Just in Time Software](https://commaok.xyz/ai/just-in-time-software/) which is basically the idea of using an LLM to create on-demand software.
I'm not _quite_ there yet but the post did illustrate to me that the barrier to creating software is now _really_ low and so ideas that used to be "cool but you know... I'll never get around to it" are suddenly an hour or two to get a working prototype.
People in my life have been saying they use LLMs to do this for _years_ but especially early on when the models sucked for coding I just wrote it off as bros being blinded by hype.
Maybe I'm blinded by hype now too... I am going to choose to ignore that intrusive thought and move on with the post.

> Seriously though, I promise I have been a skeptic of AI since the beginning.
> I don't take any claims at face value and resisted using it until it became a "not a mandate but please please please use this we'll pay for it" policy.
> I've been cautiously adopting LLMs for work and scrutinizing everything they do that touches production, even when they're good, I take seriously signing off on any work they do as if it were my own.

So I started my first Claude Code Vibe-project: editor.elijah.run.
I had a clear vision for how _I_ would implement it, so I wrote that up, paid for Claude Pro, and had it spin it's wheels for a few minutes coming up with a plan.
The first iteration was good, but not perfect so we iterated over the next few days until I got it good enough to write a few of my Backlog posts in it.
Check it out for yourself, the read-only view covers about half of the features, the publishing side of things being the other half.

My workflow turned out to be _similar_ to that described in [this post](https://boristane.com/blog/how-i-use-claude-code) except I really like to fully break up planning and execution, so I get Claude to use `beads` to create tasks from Claude's plans, then clear the context and pick tasks off the stack in priority order.

Well... I should say I _used_ beads.
I got frustrated with bead's overly-perscriptive workflow requring pushing freqently and having it's cache break periodically requiring me to sync and get a bunch of duplicate tickets -- I was certaintly doing something wrong but... I built my own beads called `nbd` (`n`ot `b`ea`d`s).
I know... [the sideprocalypse](https://johan.hal.se/wrote/2026/02/03/the-sideprocalypse/) comes for us all.

I plan to use claude to make a few other hobby projects including a database of quotable things people say, which is an idea I've literally had for 10 years but just never got around to.

Notably I probably _won't_ use it for games development, at least not in the same "vibe-coding" style.
Maybe for code review and learning hard concepts like shaders.

# Can you philosophise about generative AI ethics for a bit?

I thought you would never ask.

This section is last because it holds essentially no value for most readers.
It's basically me writing down my thought process, drawing lines in the sand about what is a "good" use of generative AI and what is a "bad" use of it.
You of course can do whatever you like, but I am using this post as an opportunity to help articulate my thoughts on the matter since every time it comes up I find myself stumbling over ideas until I just sorta... shrug.

I am not an ethisist, nor am I a philosopher.
I took two ethics/philosphy-adjacent courses in undergrad and that's about it.
I watch nerd youtube that occasionally covers this stuff, but I wouldn't say I am qualified to talk about this more than your average bear.

With that out of the way...

## Let's talk about disclosure

I hate seeing "AI Art" that isn't tagged.
Or reading a blog post "totally written by a person" but clearly isn't.
Or a Pull Request with a description longer than the declaration of independence that the author clearly hasn't read but doesn't mention it was "Created with Claude Code".

My golden rule is:
> Always disclose your use of AI.

Every time I use an AI I make that very clear and I am very up-front.
I co-author commits, I tag blog posts, I mention it in slack messages at work.
I hate taking credit for what the bots did.
I can't articulate _why_ I think you should do the same, but I think you should also tag everything a bot touches with your name on it.

If I smell AI and you don't disclose it, I will judge you.

## When should I use Generative AI?

Maybe ironcailly, since I am a "coder" by trade, but I think using LLMs to generate code is great.
It is democratizing of this technology that has been largely gate-kept for many decades.
The barrier to entry to code is super high and vibe-coding is a way to unlock the super-power that is casting spells on a CPU without needing to care what the fuck a mutex is.

Even as a coder it's great.
I can focus on the skills I want to develop, like games programming, and let it create things in my hyper-perscriptive way like using Rust for frontend development because Rust!

I also think it's a great code reviewer.
It's not better than a human, but it's a great first-pass that catches obvious issues.
It's like
> Do the tests pass, is the linter happy, and does the bot think it looks good?
> Great now ask Joey for feedback.

In fact at my current job we have [bots review _and approve_ changes](customer.io/learn/how-we-work/how-we-taught-ai-to-approve-pull-requests) which works... way better than you would expect.

## When should I **NOT** use Generative AI?

Pretty much the rest of the things.

First of all, don't use it for anything you _need_.
Or if you do, make sure it's making the same decisions you would do and you're just using it as a tool to accelerate development.
I am horrified at the idea of generating tons of software that I depend on for my income and suddenly my coding agent of choice is taken away, I am priced out, or some third thing happens that means I am unable to use them any more.
At least if it's making sensible design choices I _can_ maintain the softare it creates, just much much slower.

Generating images, videos, audio, written pros -- it's all super icky!

If you do use Generative AI and put your name on it, just for the love of god tag it as ai-generated.
I think people are scared to do that, but it errodes trust when we have to constantly be on our toes about "Wait is this AI?"


