+++
title = "vibing"
date = "2026-02-21"
description = "i start hanging out with some robot named claude"
taxonomies.tags = ["gen-ai"]
+++

so i started vibe coding and i want to share some fun projects i--

> this feels a bit like back in 2023 when youtubers and podcasts all came out with some variation on "chatgpt wrote this video".
> novel and interesting at the time, cliche in retrospect.
> it feels like every time i refresh hacker news i see a new post about somebody discovering that "hey these robots aren't so bad at coding, the thing i've historically prided myself on being really good at. weeeeeeee"

so yeah i started vibe coding.
or maybe we're calling it pair-programming with an llm.
or maybe it's clicking accept the plan and reviewing the output.

![credit: some reddit post i saw](1000008082.jpg)

# enough chit chat what did you build cyborg?

a few things!

## blog editor

i like my website.
i like writing, curating lists and collections, it's a good creative outlet.
i especially like that it is a static site hosted for free on github pages.
i don't need to pay a monthly hosting fee, i don't need to upgrade a database, it's free costing just my time which honestly i'm not spending very well anyway.

but i do wish i had a web editor sometimes.
because my site is a static pile of html, generated from a different pile of markdown, i really need to sit down at a keyboard to write anything.
historically if my screen doesn't have a git client and a text editor, i can't write a blog post.

but if i had a web editor i could blog _on the go_.
finish a book, blog about it.
have a genius idea for the next billion dollar rust library, blog about it.
notice a typo in my last blog post, blog about it. 

now that code is cheap, i _can_ build such an editor, so i did, and you can find it at [editor.elijah.run](https://editor.elijah.run).
the source code lives with the blog at [github.com/pop/elijah.run/tree/source/editor](https://github.com/pop/elijah.run/tree/source/editor).

the front-end is written in rust with the yew framework, and the back-end... well there really isn't one!
it all uses the github api to create and update branches.
there is a small cloudflare worker that does a github oauth token exchange, but that's basically it.

## not beads (nbd)

the next thing i worked on was trying to use [beads](https://github.com/steveyegge/beads), but hitting a bunch of problems with state get out of whack despite following their very prescriptive workflow.

so i tasked claude with building my own **not** _beads_.
I named `nbd` which i later learned is already a thing for **network block devices**.

thankfully a few days later i learned about [beans](https://github.com/hmans/beans), which is a great name and honestly was exactly what i was looking for.
much more friendly with local-first development, tickets were markdown so you can read and edit them by hand, so i scrapped `nbd` and switched to beans.

## vibebooks

i started using claude to generate custom educational materials, specifically focused at teaching tech concepts hands-on with rust.
i don't think they're _great_, i don't expect them to replace the best professors i've had, but they can certainly replace the _worst_ professors i've had.

instead of hoarding these 6/10 quality resources, and so i can read them on my phone, i published them to [vibebooks.elijah.run](https://vibebooks.elijah.run/) using [mdbook](https://rust-lang.github.io/mdBook/).

## quotesdb

a project i've wanted to do for a while is have a site that i can write down fun quotes.
not like winston churchill quotes but like your friend drunkenly saying "i peak every day" or my kid saying "pinecone apple" instead of pineapple.

so yeah, you can find that at [quotes.elijah.run](https://quotes.elijah.run).

# so uhh... how's it going i guess?

pretty good.
i only got into this because my work has _strongly encouraged_ use of llms, and by strongly encouraged i mean our gcp vertex bill is insane.

that said i use a personal claude pro account which i definitely use enough to justify the $20.
there are pretty low daily/weekly limits, compared to my work usage, but still plenty to get shit done.
all of the above projects were done in under a month!

one weird realization is how much i trust Claude Sonnet 4.6 and Opus 4.6; it's a lot.
i almost never look at the code and often don't even check functionality until a handful of features/bugfixes have been checked off and i'll validate them all in one go.

i am a picky dude, so instead of running on _pure vibes_ i'm pretty hands-on in the design and technology choice phase.
* everything is written in rust, even the front-end.
* apps should run locally, use sqlite, have thorough tests.
* everything is hosted on cloudflare using opentofu.
* everything has a [beans] ticket that is updated often.

the way i see it if i get priced out and i _have_ to pick these up projects to maintain them i don't want to learn react and python, i'd much rather be "forced" to learn some rust frameworks and continue to host my apps on cloudflare's pretty generous free-tier.

## isn't ai unethical? what's your stance?

ah thanks for teeing me up h2.

so yeah i do think generative ai is unethical for a lot of reasons, which is why i don't use most of it outside of work.

* i don't generate pros, this blog is free of LLM writing unless otherwise stated.
* i don't generate videos, images, and music.
* i don't intend to write videogame software with LLMs.

a few guiding principles i am trying to follow:
1. disclose any use of generative ai.
2. if, pre-generative-ai, you would have paid somebody to make this, you should.
3. don't build an ai tool, use ai to build a tool.
4. don't build your livelyhood around these tools.

disclosure is important to me.
i am very up-front about my use of llms both at work and in personal projects.
every commit is co-authored by claude code with a note about the model used.
any image i generate i note that the image is ai-generated so there is no ambiguity.

as for paying artists to do their work, that just feels right.
why would i listen to ai music if i could listen to good, real, music?
and for making a game, i could easily generate a bunch of low-effort assets, but then that's robbing an artist of a job and more importantly it's robbing the game of the outcome of the creative process of people working together -- interesting stuff happens when people with different experiences and backgrounds make stuff!

finally 3 and 4 feel closely related.
there is a ton of hype around ai, and if i wanted to con some investors into giving me some money i'm sure i could sell my soul for a sweet pitch deck but that's incredibly boring.
just make a dumb tool that costs $0 to host on a free tier, or on your laptop or your home lab or whatever and you'll be happier in the long run.
and for the love of god don't become dependent on these things for your income -- these $20/month prices won't last long i guarantee it and when you get priced out it'll be _so so so_ painful to either pay or have to learn how your spaghetti code-base works to keep things running. 

these projects i made above either wouldn't have happened or would have taken me much longer to do without an llm, so i definitely don't feel like i am robbing anybody of a job.
i also don't have the capacity, between family, work, friends, and exercising to learn how to write front-ends in rust.
i'm focused on making games in rust, and anything outside of that feel like a waste of precious free time.

---

so yeah.
i'm using claude code to write some little hobby projects.
doing my part to contribute to the side-procalypse ;-)