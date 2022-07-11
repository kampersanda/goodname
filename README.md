# Goodname

Goodname is a tool to assist you with cool naming of your methods and software.
Given a brief description of your method or software,
this tool enumerates name candidates forming subsequences of the description (i.e., *abbreviation*).

For example, given description "Character-wise Double-array Dictionary" of your software,
this tool will suggest some name candidates such as "crawdad" and "cheddar" that are contained in a word dictionary.

## CLI tool

`goodname-cli` provides a CLI tool of Goodname.
The arguments are
- `-w`: Input word list (must be sorted, unique, and not store upper-case letters),
- `-i`: Input description of your method or software, and
- `-k`: Top-k to print (default=30).

An example usage is as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -i "Character wise Double array Dictionary" -k 10
Matched 5 candidates
1: crawdad (score=2208)
2: cheddar (score=1862)
3: chaddar (score=1830)
4: caddo (score=1584)
5: caddy (score=1569)
```

Set upper-case letters in the input description so that an output always candidate contains the subsequence consisting of those letters.
In the above example, subsequence ('C', 'D'. 'D') is always contained in the candidates.

If you obtain too many or too few candidates, adjust the capitalization setting, as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -i "Character wise double array dictionary" -k 10
Matched 378 candidates
1: crawdad (score=2208)
2: ciardi (score=1952)
3: charadrii (score=1920)
4: chard (score=1920)
5: cheddar (score=1862)
6: chaddar (score=1830)
7: chiseled (score=1776)
8: characid (score=1776)
9: cedarn (score=1736)
10: charity (score=1729)
```

The candidates are printed in score order.
The scores are assigned based on the following ideas:

- The more forward letters of each word in a description, the higher the score.
- The more letters matched, the higher the score.

Here, "word in a description" indicates space-separated ones.
If you replace space letters into other ones (e.g., hyphens), resulting scores will be changed, as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -i "Character-wise double-array dictionary" -k 10
Matched 382 candidates
1: chaddar (score=29280)
2: cheddar (score=28769)
3: charadrii (score=28688)
4: chadic (score=26632)
5: crawdad (score=26128)
6: caddo (score=25344)
7: caddy (score=25104)
8: chord (score=24592)
9: characid (score=24324)
10: charade (score=24320)
```

## TODO

 - Add Web interface
 - Devise more reasonable scoring.
 - Prepare richer word dictionaries.

## Credits

`wordlist/words.txt` is a list of English words parsed from [WordNet 3.0](https://wordnet.princeton.edu/license-and-commercial-use).

