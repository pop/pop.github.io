+++
title = "Graphics Coding"

date = "2021-04-19"

description = "Taking a complete departure from my backend web knowledge to learn how to make pretty pictures with pixels."

taxonomies.tags = [
    "graphics"
]

draft = true
+++

------------------------------------------------------------------------

It dawned on me recently that I *really like playing with computers*. I
like typing stuff into a computer and making it do exactly what I want.
I like infrastructure problems where I can write software that
orchestrates hundreds of programs working together.

I also like playing games, but I like thinking about how they're made
even more. I enjoy talking with friends about balance in games,
mechanics in games, and potential changes to those mechanics.

If you know me you might know that I'm not *that* good at computers *or*
videogames. I usually play to my strengths in both areas. With computers
I write server-side or infrastructure software, and with videogames I
play the campaign.

My friend Sam and I started playing games recently. He's based in
Minnisota, and I'm based on Portland, so we would jump on Discord every
Sunday to play games for ~2hrs. First we played Diablo 3, DOTA
Underworlds, then the Halo series, then Minecraft, and now Overwatch.
I'm a big fan of Overwatch, and I am very bad at it -- but I'm showing
steady improvement!

Overwatch is the first online competitive games I've gotten serious
about. Every other online game I played -- mostly Team Fortress 2 -- I
just played for fun and didn't think about the strategy of. With
Overwatch I am focusing on the Tank class, **studying** how to play each
character, and I feel *kinda bad* when I lose.

I'm having a blast. I am loving being bad at something.

While Sam and I were playing we started shooting the shit about making a
game ourselves. We workshopped some ideas and found this idea where
you're a cab driver and when you pick up patrons the game changes and
"glitches". You pick up a boxer and other cars start punching you, pick
up a toddler and the world is drawn in crayon, shit like that.

At a similar time I started watching Johnathan Blow and Casey Moriarti's
streams about videogame development. These guys both code their own game
engines from scratch, and they make it seem hard but fulfilling. They
don't aspouse to saying you "have" to code you own games from scrach,
but they make a lot of good points about having control over the engine,
and gaining expertise that is often lost in modern software development.

At the same time *again* (I know this is a bad story, I'm just telling
you shit is happening, we'll get to the graphics stuff in a bit) I was
working on a software project that was useful, but not concerned with
performance at all. I thought this was isolated to our company and our
project, but as I listened to these streams I was comforted knowing that
*lots* of software is written with a focus on features, and "features"
rarely include performance requirements beyond "fast enough".

In talking to some technologists in my life this is a hard idea to
articulate, but the thrust of what I'm realizing is that I don't know
how to write fast software, only "fast enough" software. I have spent my
entire career thinking of performance requirements and not thinking
about performance *possibilities*. Chandler Caruth also has good talks
on this topic.

The games industry, based on my research, makes software that is as fast
as *possible*, not as fast as *enough*. This happens to be because
making software go as fast as possible *is* a *user requirement*, but
that's still cool!

So what does it mean for software to go *as fast as possible*. ..
measuring performance .. cache hits/miss .. data locality ..
parallelization .. minimize memory allocations .. no RAII

So this is all to say that I'm using this transitive frustration from
the folks I'm following online to get into game development.

Some people might say to play to your strengths to maximize your
profitability and skills in the workplace. I agree with that, if I were
maximizing my life for money. I have decided that I am maximizing my
life for daily average joy. This means work should be enjoyable, but it
also means persuing whatever hobby brings the most joy or endorphis, not
the hobby that brings in the most cash money. I also think people that
are soly focused on one thing for that reason are boring as hell. If you
do one thing becuase that's the most logical way to maximize personal
profits, that's boring as hell. If you do one thing becuase that brings
you the most pure joy and you hadn't even thought of brancing out
because you were having so much fun -- hell yeah keep riding that high
dude.

Ultimately I like the open source Rust engines, but I don't have any
prior knowledge developing engineso I can't be more than a user. But I
want to be a contributor. I want to be the kind of expert that is able
to make Rust a games programming language. I want to be the kind of
expert that gives talks on how to write performant code and inspire
others like I have been inspired.

I'm not going to quit my job just to persue this, I've flaked out of
other ideas like this in the past, but it is my main hobby at the
moment. If it becomes my passion maybe I will persue it to try to make a
living off of this notion.
