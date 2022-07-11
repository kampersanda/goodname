# Goodname

Goodname is a tool to assist you with cool naming of your methods and software.
Given a brief description of your method or software,
this tool enumerates name candidates forming subsequences of the description (i.e., *abbreviation*).

For example, given description "Character-wise Double-array Dictionary" of your software,
this tool will suggest some name candidates such as "crawdad" and "cheddar" that are contained in a word dictionary.

## CLI tool

`goodname-cli` provides a CLI tool of Goodname.
The arguments are
- `-w`: Input word list (must be sorted, be unique, and include no upper-case letters),
- `-i`: Input description of your method or software, and
- `-k`: Top-k to print (default=30).

An example usage is as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -i "Character wise Double array Dictionary" -k 5
Matched 10 candidates
1: crawdad (score=2208)
2: chided (score=2064)
3: cheddar (score=1862)
4: creeded (score=1684)
5: cradled (score=1680)
```

Set upper-case letters in the input description so that an output candidate always contains the subsequence consisting of those letters.
In the above example, subsequence ('C', 'D'. 'D') is always contained in the candidates.

If you obtain too many or too few candidates, adjust the capitalization setting, as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -i "Character wise double array dictionary" -k 5
Matched 1047 candidates
1: crawdad (score=2208)
2: chresard (score=2116)
3: chided (score=2064)
4: cardita (score=1988)
5: chawdron (score=1976)
```

The candidates are printed in score order.
The scores are assigned based on the following ideas:

- The more forward letters of each word in a description, the higher the score.
- The more letters matched, the higher the score.

Here, "word in a description" indicates space-separated ones.
If you replace space letters into other ones (e.g., hyphens), resulting scores will be changed, as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -i "Character-wise double-array dictionary" -k 5
Matched 382 candidates
Matched 1047 candidates
1: chided (score=28932)
2: cheddar (score=28769)
3: charadrii (score=28688)
4: chudic (score=28672)
5: cradled (score=26880)
```

## TODO

 - Add Web interface
 - Devise more reasonable scoring.
 - Prepare richer word dictionaries.

## Credits

`wordlist/words.txt` is a list of English words provided by [dwyl/english-words](https://github.com/dwyl/english-words).
