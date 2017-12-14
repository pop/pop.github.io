How to Get the Most out of Your Python Exceptions
=================================================

:date: 2016-04-03
:slug: python-exceptions
:status: published
:summary: One of the most useful features of Python are exceptions-- but how the heck do they work? Let's find out.
:tags: Technical, Pythonic, Guide

.. code-block:: python

    var = input('Please enter a number not equal to 0: ')
    try:
        var = 1/float(var)
    except ZeroDivisionError as e:
        print('Error: ' + str(e))
        print('You had one job!')
      

.. warning:: 

    This post does not cover *what* python exceptions are. If the above
    code doesn't make sense you should check out this `Introduction to Python
    Exceptions`_ from wiki.python.org before reading this post.

If you've fallen in love with Python you've no doubt discovered exceptions:

.. code-block:: python

    try:
        # do a thing
    except:
        # thing did not work,
        # do something else

They are endlessly useful and help one to avoid writing checks upon checks upon
checks before getting to the meat of your project. Just try a thing, catch the
error, keep on rolling (or fail gracefully).

As powerful as they can be, I have found a lot of folks (past me included) who
don't know how to find python exceptions and don't know how to write their own
exceptions! So let's do that.


Finding Exceptions
------------------

When writing exception handling code it's kosher to explicitly state *which*
error you expect.

.. code-block:: python

    # Bad: 
    try:
        # A thing that might not work
    except:
        # Something else

    # Good:
    try:
        # A thing that might not work
    except SpecificError as err:
        # Maybe print(err)
        # Something else

When I first found this out I thought *Golly that does sound useful; I always
try to be explicit in my error handling -- but how??* Thankfully future me is
here to answer questions like this.

The exception you are looking for (for instance, ``SpecificError`` in the above
pseudo-code) can be found in the python traceback:

.. code-block:: text

    $ echo "open('myfakefile.txt', 'r').close()" > my-unhandled-script.py
    $ python my-unhandled-script.py
    Traceback (most recent call last):
      File "/tmp/test.py", line 1, in <module>
        open('myfakefile.txt', 'r').close()
    IOError: [Errno 2] No such file or directory: 'myfakefile.txt'

That bit on the last line ``IOError`` is the exception you're looking for. So
when you write your code you'll say something like the following:

.. code-block:: python

    try:
        open('myfakefile.txt', 'r').close()
    except IOError as e:
        print(e)
        print('File `myfakefile.txt` does not exist')

To recap, here is one way (and my preferred method for) 'doing' python
exception handling:

#. Write breakable code.
#. Run breakable code, see what exceptions python spits out.
#. Wrap breakable code in explicit ``try/except`` blocks.
#. ???
#. Profit.


Writing Your Own Exceptions
---------------------------

You (who me?), yes *you* can write custom python exceptions. It's dead easy
too.

The long and short of it is you define an exception class which either inherits
from the ``Exception`` class or another pre-existing exception.

.. code-block:: python

    class CusssstomError(Exception):
        '''Raise when snakes'''
        def __init__(self, message):
            self.message = message

    def check_for_snakes(foo):
        if 'snake' in foo.lower():
            raise CusssstomError('Snakes! I hate snakes!')

    try:
        foo = input("Just don't mention snakes: ")
        check_for_snakes(foo)
    except CusssstomError as e:
        print(e.message)
    else:
        print("Thanks. I appreciate it.")

The above code defines the ``CusssstomError`` exception class which you can
``raise``. Very neat and `pythonic`_


Further Reading
---------------

Here are a few references I suggest you check out to get an even better grasp
on exception handling and custom exceptions:

* This Stack Overflow post: `Proper way to declare custom exceptions in modern
  Python?`_.
* This Python doc: `Built-in Exceptions`_.

And as always, search engines are your friends.

.. _Introduction to Python Exceptions: https://wiki.python.org/moin/HandlingExceptions
.. _pythonic: https://docs.python.org/2/glossary.html#term-pythonic
.. _Proper way to declare custom exceptions in modern Python?: http://stackoverflow.com/questions/1319615/proper-way-to-declare-custom-exceptions-in-modern-python
.. _Built-in Exceptions: https://docs.python.org/2/library/exceptions.html
