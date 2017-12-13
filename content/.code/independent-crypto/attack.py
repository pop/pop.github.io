# attack.py
from time import time # Used to get a timing difference.
from string import ascii_lowercase # All lowercase characters
from string import digits # All digits as strings
from subprocess import call # used to exec a secret.py
from os import devnull  # Suppress output of secret.py

current     = list('aaaa')  # Initial guess
characters  = ascii_lowercase+digits # All possible characters in the secret

# We know the string is the same length as our initial guess
for i in range(len(current)):
    guess_times = [] # Keep track of execution times

    for x in characters:
        current[i] = x # Swap the current letter with the current guess

        # Uncomment the following line for fun debug output
        # print('Making guess {}'.format(''.join(current)))

        # Execute `secret.py` and time it
        start = time()
        a = call(['python', 'secret.py', ''.join(current)], stdout=open(devnull, 'wb'))
        end   = time()

        # Add that time to the list
        guess_times.append(end-start)

    # Uncomment the following line for fun debug output
    # print('max {} min {}'.format(max(guess_times), min(guess_times)))

    # This is a hackey-looking way of getting the outlier time.
    current[i] = characters[guess_times.index(max(guess_times))]
    print('character {} is {}'.format(i, current[i]))

    # Uncomment the following line for fun debug output
    # print(guess_times.index(max(guess_times)))

print('Final guess is {}'.format(''.join(current)))
