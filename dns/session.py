import random
from functools import lru_cache


@lru_cache
def get_secret():
    """Load the server-side secret from disk."""
    with open('secret.bin', 'rb') as o:
        return o.read()

class Session:
    """
    This represents the session of a student taking the exam,
    and their state of the world.
    """
    def __init__(self, seed=b''):
        self.seed = get_secret() + seed

    def gen_by_name(self, name: str):
        """
        Produce a random number generator for a particular name.

        This generator is initialized with the session's seed, and the given name.
        Thus, you can use this to generate consistently random values for a particular job.
        """
        gen = random.Random(self.seed + name.encode())
        return gen
    