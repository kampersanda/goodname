# Goodname

Goodname is a tool to assist you with cool naming of your methods and software.
Given a brief description of your method or software,
this tool enumerates words forming subsequences of the description.

For example, when a description of your software is "Character-wise Double-array Dictionary",
this tool will suggest some name candidates such as "crawdad" and "cheddar" that are contained in a word dictionary.

## CLI tool

`goodname-cli` provides a CLI tool of Goodname.
The arguments are
- `-w`: Input word list (must be sorted, unique, and not store upper-case letters),
- `-i`: Input description of your method or software, and
- `-k`: Top-k to print (default=`30`).

An example usage is as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -i "Character wise Double array Dictionary" -k 10
Matched 5 candidates
0: crawdad (score=60)
1: cheddar (score=51)
2: chaddar (score=50)
3: caddo (score=41)
4: caddy (score=37)
```

Set upper-case letters in the input description to contain the letters in output candidates.
In the above example, subsequence ('C', 'D'. 'D') is always contained in the candidates.
If you obtain too many or too few candidates, adjust adjust the capitalization setting.

The candidates are printed in score order.
The scores are assigned based on the following ideas:

- The more forward letters of each word in a description, the higher the score.
- The more letters matched, the higher the score.

Here, "word in a description" indicates space-separated ones.



## TODO

 - Add Web interface
 - Devise more reasonable scoring.

## Credits

`wordlist/words.txt` is a list of English words parsed from [WordNet 3.0](https://wordnet.princeton.edu/license-and-commercial-use).

