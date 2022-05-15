Buying a House Part 4: Is this investing?
=========================================

:date: 2022-01-18
:slug: buying-a-house-04-is-this-investing
:status: published
:summary: Everybody says it is, so I guess it is?
:tags: housing, life, HGTV

*This is part 4 of a 4 part series on buying a home*.
*Check out part 1 for to start from the beginning*.

Before buying a house a major talking point used to convince me of the idea was some variation of "It's a great investment!" or "You're just throwing money away by renting".
I try to have a healthy level of skepticism about things, especially when counter-arguments aren't part of most the usual talking points on a topic.
So let's figure out "is buying a house a good investment?"

"Houses are a great investment" origin story
--------------------------------------------

Before de-bunking -- or just bunking -- this myth, let's ask where it came from.

Hundreds of years ago, when you had to own property to be considered a voting citizen, investing in property was a great idea!
Housing was the most affordable asset you could buy that could passively grow in value.

As the years went on this continued to be true.
Housing is a market rarely which declines value, with umm... one big exception, and it can easily be passed on to family, creating generational wealth.

This notion also pre-dates easier market investing tools like the index fund, Vanguard, and Robinhood.
When it wasn't really clear how an average individual could get a diversified portfolio of stocks and bonds, a house was a safe investment that was all but guaranteed to increase in value -- and it would protect you from snow and rain and shit!

So the origin story of this mantra isn't *false*, but it may be outdated.
Let's find out.

Meet Finley
-----------

For the purposes of this post we will invent a hypothetical protagonist named Finley.
Our fictional mascot was created in a lab to be incredibly average: the most basic curve-fitting person we could find in the the Portland tri-state area.
Finley likes all the Marvel movies, drinks IPAs, and they go on hikes every other weekend.
They also make an average salary, live in an average apartment, and intend to buy an average house.
How convenient for talking about hypotheticals.

Finley lives in Portland, so all of our numbers are for Portland Oregon, USA.

First Finley rents an apartment
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Finley is currently renting an apartment.
On average this is $1250 for the Portland area.
This is roughly equivalent to what you would pay for a mortgage on a 300k house with a 20% (60k) down payment and a 3.0% interest rate (which is only a little optimistic).

Here's what I used to cook up those numbers:
* https://www.mortgagecalculator.org/
* https://www.realtor.com/mortgage/rates/Portland_OR

Then Finley buys a house
~~~~~~~~~~~~~~~~~~~~~~~~

In 2020, when I bought a house, the median house prices ranged from 430k to 500k trending upward.
In the last 12 months from writing this post median home prices have ranged from 469k to 550k with a trend still going up.

https://www.redfin.com/city/30772/OR/Portland/housing-market

For simplicity let's say Finley got a deal and bought a house for 480k with a 3.0% mortgage rate.
Their down-payment is 96k and their monthly mortgage payment is $1,900.

Right off the bat Finley is paying 650 more per month on top of the 96k they had to pay up front.
Let's assume Finley is still able to cover this cost since they're very frugal and despite cost of living going up 34% they are still spending less than 1/3 of their income on living expenses.

How money works for you
-----------------------

What has Finley *financially* gained and lost by buying a house?

Finley has *lost* the ability to invest a lump sum of 96k on top of an extra 600/month which could be going toward a range of investments.
Assuming Finley would have invested this in a stock market index we expect this to have increased in value by about 10% year over year given historical trends.
To put numbers to these claims: 96k alone would have appreciated in value to 1,600,000 over 30 years, the duration of most home loans.
If we add the additional 600/month we would have about 2,860,000 after 30 years.

(these are very rough calculations using this python code)

.. code-block:: python

	a = 600 * 12  # Annual investable income
	s = 96000     # Starting seed fund
	for x in range(30):  # Collect 30 years
	  s *= 0.1           # Grow by 10% for the year
	  s += a             # Add money saved by renting

(this is a very rough approximation of collecting interest over 30 years)

Note this doesn't mean he are guaranteed to have 2.9 million at the end of 30 years; some of those gains would be diminished by inflation, rent would go up, and the market could have a down-turn at the 30 year mark.
Plus, Finley would probably spend more as they get older (called "lifestyle inflation") resulting in diminished invest-able funds -- but they would also probably get cost of living raises over time, so this should all come out in the wash.

But a house would *also* grow in value right?
This is tricky to calculate because we're in a weird housing market right now -- don't call it a bubble -- but let's take the last 10 years at face value.
In 2011 the median Portland house was ~260k, and today it is ~550k.
This is a 111% return over 10 years, or 7.82% annually.

So if Finley sold their house after 10 years they would make a profit of ~290k.

I neglected to calculate what Finely's index fund strategy would return after only 10 years, so let's do that now.
Using the above code, but limiting it to just 10 years instead of 30, we get a ~363k total which does beat the housing investment by a fair margin. `citation needed <https://www.calculator.net/roi-calculator.html?beginbalance=259000&endbalance=550000&investmenttime=date&investmentlength=2.5&beginbalanceday=08%2F01%2F2011&endbalanceday=08%2F01%2F2021&ctype=1&x=69&y=36/>`_

Note that I am omitting all of the nickles and dimes that add up on both sides.
Capital gains tax makes your index fund strategy less appealing.
Home ownership means you are on the hook to fix anything that breaks, pay property tax, and there are fees associated with buying and selling a home -- plus the cost of the loan which we didn't touch on at all!
At the same time, all the money Finley was paying in rent really was being flushed away and not appreciating in value, which is a shame.

Elijah, just tell me if it's good or bad
----------------------------------------

A house is not a great investment, but that's not the point.

I didn't buy a house because it was a good investment, I bought a house because I wanted more space and it penciled out to about the same monthly cost as my super expensive bougie central-Portland apartment.
I am glad that my house will re-coop some cost when I decide to move out, and I do like the stability of home ownership; I don't need to worry about rent going up, or getting evicted because my landlord wants to demolish the building for something else, etc.

So many variables go into the "is this a good investment" question.
If you need to stretch yourself thin, or you already have debt, or what the monthly cost is -- all of these and more contribute to the viability of buying a house.
If your monthly housing costs wouldn't go up, it's probably a solid move.

Home ownership is not perfect, it's not a great investment, but it is an OK use of your money if you can afford it.
It's financially sound if you can make it work, comfortably, but it's not your only option.
Being frugal and renting an affordable apartment allows you to make other "better" investments, if that's all you're min-maxing for.

I don't regret buying a house (yet?) but I would be just as happy if I was renting a house, or still living in an apartment.

One more soap-box
-----------------

Side note: we need to build more housing.
Portland passed a bill recently allowing more dense housing.
If I had more money I would throw money at building triplex housing in Portland.

For too many of my 20-something peers the problem is not "If I want to buy a house" but "When can I afford to buy a house?".
There is a bit of a market boom right now, but with more housing the market should settle down.
In a perfect world anybody would be able to buy property if they want.

----

Thank you for reading this fourth and final installment of my housing series.
Now go out a buy a house!

Lol, just kidding.
Go do more research you crazy kid.
