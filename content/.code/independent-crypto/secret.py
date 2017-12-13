# secret.py
from time import sleep # Used to exaggerate time difference.
from sys import argv   # Used to read user input.

def is_equal(a,b):
    """Custom `==` operator"""
    # Fail if the strings aren't the right length
    if len(a) != len(b):
        return False

    for i in range(len(a)):
        # Short-circuit if the strings don't match
        if a[i] != b[i]:
            return False

        sleep(0.15) # This exaggerates it just enough for our purposes

    return True

# Hard-coded secret globals FOR DEMONSTRATIONS ONLY
secret = 'l33t'

# This is python for "If someone uses you as a script, do this"
if __name__ == '__main__':

    try:
        # The user got it right!
        if is_equal(str(argv[1]), secret):
            print('You got the secret!')

        # The user got it wrong
        else:
            print('Try again!')

    # The user forgot to enter a guess.
    except IndexError:
        print('Usage: python secret.py yourguess\n' \
             +'The secret may consist of characters in [a-z0-9] '\
             +'and is {} characters long.'.format(len(secret)))
