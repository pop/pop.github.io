+++
title = "Circular Imports"

date = "2021-01-31"

description = "Nothing is that simple..."

taxonomies.tags = [
    "python"
]
+++

Don't you hate when you're writing a python program and you get hit with one of these?

```txt
Traceback (most recent call last):
  File "a.py", line 1, in <module>
    from b import g
  File "/home/pop/Projects/src/localhost/circular/b.py", line 1, in <module>
    from a import f
  File "/home/pop/Projects/src/localhost/circular/a.py", line 1, in <module>
    from b import g
ImportError: cannot import name 'g' from partially initialized module 'b' (most likely due to a circular import) (/home/pop/Projects/src/localhost/circular/b.py)
```

What a drag.

Circular imports are of course the literal devel in programming, but in a Godel Escher Bach sort of way they are all around us.

## welcome to my ted talk

There are a lot of problems in the world that everybody agrees are problems[^1], but we can sometimes disagree about what the solution is.

Many of these problems stem can be thought of as emergent properties of other phenomena.

Take for example, plastic.

## fuck plastic

Remember the good old days in America when women tended to the house while PTSD ridden white men worked a 40 minute drive away and the ethnically, racially, sexually, abally discriminated minorities were repressed?

Well something that actually was pretty sweet was milk delivery.

Milk, delivered regularly in reusable glass bottles had incredibly ecological benefits compared with how we get milk[^2] today.

Deliveries are an incredibly efficient means of distriuting goods[^3] and re-using glass bottles requires far less energy than say, melting a bottle down or worse creating a plastic single-use bottle.

I really want to focus on the glass bottles because they are *so* efficient, why did we ever stop?

Lobbying.

## from plastics import lobbying

That's right: lobbying, not the free market, gave us the fucking horrific amount of single-use plastic we have today.

For sure there are a million things we need to cut the fuck out if we want this planet to be anything close to what we were born into in 100 years, but plastics are worth spending a few hundred words.

The plastics industry spent millions[^4] lobbying to allow the use of single use plastics instead of reusable containers.

Instead of identifying the inevitable crisis of too much fucking plastic and shutting that shit down, governments across the world wer just like "yeah this is cool, pour up my dude".

Congress, given no external input, very well could have legislated against the use of single-use plastics, but because they were convinced... with money... otherwise here we are in a world full of single use plastics, and just like a virus it's spread so far that it feels impossible to slow it's spread.

So clearly we need to stop lobbying.
We need to think of some clever way to prevent corruptable government officials from being bought off to vote against the interests of their constituents, their country, and future generations...

Well that's clearly impossible.

Let's dig deeper.
What causes lobbying?

Profit.

## from lobbying import profit

It is clear that profit is the reason lobbying exists.

Some lobbying is good. If you own a farm and you hear about legislation that will like... just straight up dump a bunch of nuclear waste on your property, you're probably going to knock on some doors and grease some palms to prevent that from happening.

Arithmetically it makes sense that lobbying is tied to profit.
If a piece of legislation is projected to decrease profits by 30 million USD, you're probably willing to spend 2 million USD to prevent it from going through.

Lobbying, like swordfishing[^5], is a targeted, and therefore monetarily effective, way to influence change.

Maximizing profit has a lot of other problems[^6] like supressing wages despite productivity going up, and cock-blocking public alternatives to private services like healthcare... Where was I?

Oh right, so this begs the question, where does profit come from?

## from profit import ???

## soapbox intensifies

[^1]: All *good faith actors* actors agree are a problem.

[^2]: Yes, also mylks.

[^3]: Citation needed?

[^4]: ... *sigh* citation needed.

[^5]: The internet phishing scam not the oceanic activity.

[^6]: ... citation... needed...?
