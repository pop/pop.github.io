Dynamic Dispatch in Python
==========================

:date: 2021-09-02
:slug: python-dynamic-dispatch
:status: published
:summary: HeY wAnT tO SeE a NEat PYtHOn TriCK!?
:tags: python, dyanmic dispatch, tidbit

🦝 Hey want tO see a nEat PYtHoN triCK?

.. code-block:: python

    def f(name: str):
        print("hiya {}".format(name))
        
    locals()["f"]("Spongebob")

🐮 Oh cool, does that use the string ``"f"`` to find--

🦝 DoEs It FiNd ThE fUnCtIoN ``"f"`` aNd CaLl iT?? yEaH It DoEs.

.. code-block:: text

    $ python script.py
    hiya Spongebob

🐮 Oh that's neat. I didn't know you could do that with Python--

🦝 whaTEveR i dON't cArE wHaT YOU tHinK.

🐮 You asked me --

🦝 I sAiD sHuT uP. i'M dOiNg HoT gIrL sHiT.

🐮 Are you ok? You're yelling a lot about Python again--

.. code-block:: python

    # shut up

    def test_patrick(context: TestContext):
        # do thing

    def test_crabs(context: TestContext):
        # do other thing

    context = TestContext()
    tests = []
    for t in locals():
        if t.startswith("test_"):
            tests.append(t)
    for test in tests:
        test(context)

🐮 Oh so you could... 

🦝 ...

🐮 Are you going to interrupt me?

🦝 Not yet.

🐮 Ok... you could search your scope and find a bunch of functions with a name and common interface--

🦝 AND CALL THEM! DYNAMIC DISPATCH!!

🐮 That's nice.

🦝 YEAH IT IS COOL!
