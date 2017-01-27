def is_pangram(string):
    counts = get_char_counts(string.lower())
    for char in iter("abcdefghijklmnopqrstuvwxyz"):
        if counts.setdefault(char, 0) == 0:
            return False
    return True

def get_char_counts(string):
    """
    Get a dictionary with all the string characters as keys, and the number
    of times each occurs as value
    """
    counts = {}
    for char in iter(string):
        counts[char] = 1 if not char in counts.keys() else counts[char] + 1
    return counts

