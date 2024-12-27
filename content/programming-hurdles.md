+++
title = "Conceptual hurdles in programming"

date = "2016-08-21"

description = "Sage advice for myself in Freshman year."

taxonomies.tags = [
    "programming"
]
+++

I've been actively learning programming for almost five years now.
I started by taking a C++ class at Portland Community College.
That class was the best thing to happen to my engineering career because it taught me that *programming is very hard, and I am not naturally good at it*, but it was also very rewarding so I stuck with it.

I'm not special either, most of us aren't very good at programming because it's so abstract and hard to grasp, like Math and Infinite Jest.
There are of course the people that seem like *programming gods*, but nine times out of ten those people have been programming since they were eight and it's really not fair to compare yourself to them.
That's like comparing your sporting abilities to *any olympic athlete*.
Don't be so hard on yourself.

In learning to program, and now teaching others how to program, I've identified a few **hurdles** (mountains, cliffs, obstacles, w/e) that folks tend to hit and have trouble crossing.
They always get past these hurdles eventually (or they quit CS ☹), but not without a bit of unnecessary struggle.
Struggling builds character, and I don't want to cheat anybody out of that, but I do want to help those that want it.

This post will in no way guarantee your success in overcoming these obstacles but it should at least give you a head start to help you understand what you're learning and point you in the right direction when you want to ask a question.

## Data Structures

Understanding that **a lot of programming is basically manipulating data to fit a need** is pretty important.
A social network can be thought of as a bunch of people's personal information plugged into a bunch of algorithms to make connecting easier.
A word processor is really just a front-end for some [XML under the hood](https://en.wikipedia.org/wiki/Microsoft_Office_XML_formats), which is itself [just a way to represent structured data](https://en.wikipedia.org/wiki/XML).

When handling data try to keep in mind what you are trying to accomplish and the best way to structure the data in the pursuit of that goal.

{% define(word="[Arrays/Lists/Vectors](https://en.wikipedia.org/wiki/Array_data_structure)") %}
**Arrays and Lists are an ordered data type**. Use them when you need to
keep track of the order things happened in, like queuing an event.
Try to keep them *relatively small* since they are fast for retrieval but not inherently space efficient.
{% end %}

%{ define(word="[Hash-table/Key-Value/Dictionary](https://en.wikipedia.org/wiki/Hash_table)" %}
**Hashes are useful for storing unordered data** with keys, like an address book or a small database.
We use them to access data quickly and easily since data is retrieved using a key and always takes approximately the same amount of time.
When you get data out of a dictionary you provide a key and get back the associated data, just like (you guessed it) *a real dictionary* where provided the word you get back a definition.

Hash-tables are useful tools but should only be used for *relatively small* amounts of data.
There is a bit of overhead in creating the table for storing your data so if your data-set gets too big you'll run out of memory.
{% end %}

{% define(word="[Linked Lists/Sorted Trees](https://en.wikipedia.org/wiki/Linked_list)") %}
Linked Lists and Sorted Trees aren't exactly the same thing but I'm lumping them together because they both deal with a lot of the same concepts and can be implemented in similar ways.
In learning about LL/ST you'll probably deal with Structs, Nodes, Pointers, and dynamic memory allocation, which if you're like me will totally break your brain and then become second-nature.

**One advantage to these structures is that they can be implemented to take up only the space they require**.
Where Hash-maps and lists are hard to make the *right size*, LL/ST can only take up exactly as much room as they need.
The trade-off (a word you hear a lot when dealing choosing data structures) is that they're not always the *fastest* way to store/retrieve data.
Just consider your use-case and think about what you need for the task at hand.

If you're interested in learning Rust though there's a great guide on [Learning Rust With Entirely Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html).
I do suggest it.
{% end %}

**These are all just the structures you're giving your data** (a series of ones and zeros on disk and in memory).
Anybody that's done work with a sufficiently complicated project (e.g., the Linux Kernel) can tell you that one glob of data can be treated as a Linked List, Array, Hash -- or all three at the same time!
The structure you give your data is just so *you* can work with it, the computer doesn't really care one way or another, so choose what makes the most sense.

*Speaking of data structures...*

## Objects/Classes

**Objects are logical groupings of data (variables) and functions that act on that data**.
They differ from structs in that they are (usually) private by default, meaning that variables declared in an object are not accessible from the *outside* unless you explicitly say so.
Variables are usually manipulated via a method called a *getter* and *setter*.

Well... what I've actually described so far is creating a **class: aka a blueprint for an object**.
This python is a pretty succinct way to describe classes and objects:

```python
"""
objects_example.py

Declare a class named BazClass.
"""
class BazClass(object):
    def __init__(self, x=None):
        """
        The __init__ function sets class variables and sets up the object.
        """
        self.var1 = x
        self.var2 = 6

    def a(self):
        """
        a() acts on the object variables (accessed via `self`).
        """
        print("Variable you set {}".format(self.var1))
        print("Variable set by class definition {}".format(self.var2))

"""
Creating an instance of the class and call the `a()` function.
"""
foo_object = BazClass(7)
foo_object.a()
```

Objects are something that will *click* after you use the for a while.
There's some nuances and implementation quirks depending on the language you're using, but in the end they're just logical collections of private data and functions.

## Functional Programming

[Functional Programming](https://en.wikipedia.org/wiki/Functional_programming) (FP) was my first big paradigm shift in CS since functions. FP includes concepts of variable immutability (once it's set it's set), callbacks (functions calling functions and propagating results up), program state -- the list goes on. Here's an example to get us started.

Non-functional paradigm:

```javascript
function average_evens(x) {
    avg = 0;

    for (i = 0; i < x.length; i++) {
        if (x[i] % 2 == 0) {
            avg += x[i];
        }
    }

    avg = avg/x.length;
    return avg;
}
```

Functional Paradigm:

```javascript
function average_evens(x) {
    return x.reduce(function(x) {
        if (x % 2 == 0) {
            return x;
        }
    }) / x.length;
}
```

In the non-functional example one would create a variable, iterate with a for-loop, and return a variable at the end.
In the functional example you call functions which return data that you handle (callback) and that result is added to or replaces the original data. I'm honestly not doing this topic justice but it's one of those things you either learn first or you learn the hard way.

This is just the tip of the ice-berg; many people prefer functional programming.
If you are even remotely interested you should find one of those people and let them talk your ear off.

If you're learning functional programming I suggest either [Learn You a Haskell for Great Good](http://learnyouahaskell.com/chapters) if you're interested in learning Haskell (which is interesting an interesting language if nothing else) or [Functional Programming in Python](http://www.oreilly.com/programming/free/functional-programming-python.csp) for *pythonistas*.

## Frameworks

**Frameworks are a collection of libraries, methods, and tools to accomplish a specific type of task**.
If you want to get something *complicated* done *fast*, you'll use a framework.
Take for instance a web-app like Facebook or Twitter: it needs to be able to *send/receive HTTP requests*, *interface with a database* / *craft database queries*, *render web-pages*, *and* whatever the actual website is supposed to do.

Frameworks can be very small or very large but they are always a big hurdle for those of us that have never worked with one before.
A good starter framework I suggest is [Flask](http://flask.pocoo.org/) for Python-based web-apps.

## Testing

**Testing is writing a program to test your program**.
There are many different *kinds* of tests from *unit tests* (checking single functions), to *integration tests* (checking that your functions work *together*), to *random tests* (trying to break your program by telling a computer to break your functions by using they in weird ways).

Remember that first program you wrote for class?
You wrote one that prompted the user for data and then manipulated that input in some way?
Remember how you tested that?
Probably in the most tedious way imaginable: **by hand**.
Tests are much easier to write than your actual code and while they're a tedious investment up-front it's a small commitment relative to the *hours* you'd spend checking your program every time you made a change.

## Troubleshooting

The last thing isn't really about programming but about *fixing* your programming.
When you're learning a new paradigm, language, or framework you're going to spend a lot of time *fixing* what doesn't currently work much more than you're going to actually be producing working code.
This skill can also be summed up as 'How to Read Error Messages and Google well'.

My troubleshooting advice is to *read* the error message, don't give up, and search for anything that looks meaningful.
Once you find an answer try to *grok* what the answer means so you can learn *why* that worked instead of knowing *if I type this in a certain way it will not fall down*.
Future you will appreciate the investment you put into *understanding* the problem and it's solution.

Also don't be afraid to experiment. Make a copy of your program (or use [git](https://git-scm.com/)) and see if some crazy idea is exactly what you need.

------------------------------------------------------------------------

This list is far from complete but it still felt worth sharing.
If you think I missed something, [contact me](@/about.md#contact) and I might do a follow-up post.
