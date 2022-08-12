#!/usr/bin/env python3

import sys
import nltk


def get_words():
    nltk.download('words')
    return nltk.corpus.words.words()


def get_brown():
    nltk.download('brown')
    return nltk.corpus.brown.words()


def get_webtext():
    nltk.download('webtext')
    return nltk.corpus.webtext.words()


def get_gutenberg():
    nltk.download('gutenberg')
    return nltk.corpus.gutenberg.words()


def char_range(c1, c2):
    """Generates the characters from `c1` to `c2`, inclusive."""
    for c in range(ord(c1), ord(c2)+1):
        yield chr(c)


def normalize(w):
    w = w.lower()
    for c in w:
        if not c in char_range('a', 'z'):
            return None
    return w


def main():
    t = sys.argv[1][0]
    if t == 'w':
        words = get_words()
    elif t == 'b':
        words = get_brown()
    elif t == 'W':
        words = get_webtext()
    elif t == 'g':
        words = get_gutenberg()
    else:
        print('INVALID TYPE', file=sys.stderr)
        return

    print('original:', len(words), file=sys.stderr)
    converted = set()
    for w in words:
        w = normalize(w)
        if w:
            converted.add(w)
    converted = list(sorted(converted))
    print('converted:', len(converted), file=sys.stderr)
    for w in converted:
        print(w)


if __name__ == "__main__":
    main()
