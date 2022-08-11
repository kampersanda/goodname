# Goodname

Goodname is a tool to assist you with cool naming of your methods and software.
Given a brief description of your method or software,
this tool enumerates name candidates forming subsequences of the description (i.e., *acronym*).

For example, given description "Character-wise Double-array Dictionary" of your software,
this tool will suggest some name candidates such as "crawdad" and "cheddar".

## Web App

The [Web App](https://kampersanda.github.io/goodname/) is the easiest way to try this tool.
(This demo movie is at v0.1.1.)

![](./movies/demo.gif)

## CLI tool

`goodname-cli` provides a CLI tool of Goodname.
The arguments are
- `-w`: Input word list (must be sorted, be unique, and include no upper-case letters), and
- `-k`: Top-k to print (default=`30`).
- `-l`: The maximum number of arbitrary prefix letters to allow for generating recursive acronyms (default=`0`, must be in `0..=3`).

Enter your description using only lowercase letters or a space basically.
Set UPPERCASE only for letters that you want to be always included in a name candidate.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -k 5
Enter your text:
Character wise Double array Dictionary
Matched 10 candidates
   1 crawdad: ChaRActer Wise Double Array Dictionary (score=2656)
   2 chided: CHaracter wIse DoublE array Dictionary (score=2064)
   3 cheddar: CHaracter wisE Double array DictionARy (score=1862)
   4 carded: ChARacter wise DoublE array Dictionary (score=1744)
   5 creeded: ChaRactEr wisE DoublE array Dictionary (score=1684)
```

In the above example, subsequence ('C', 'D', 'D') is always contained in the candidates.
If you obtain too many or too few candidates, adjust the lettercase setting, as follows.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -k 5
Enter your text:
Character wise double array dictionary
Matched 1047 candidates
   1 crawdad: ChaRActer Wise Double Array Dictionary (score=2656)
   2 chresard: CHaRactEr wiSe double ARray Dictionary (score=2244)
   3 chawdron: CHAracter Wise Double aRray dictiONary (score=2200)
   4 chadar: CHAracter wise Double ARray dictionary (score=2176)
   5 chawia: CHAracter WIse double Array dictionary (score=2176)
```

Or, you can specify the maximum number of arbitrary prefix letters to allow for generating recursive acronyms.

```
$ cargo run --release -p goodname-cli -- -w wordlist/words.txt -k 5 -l 2
Enter your text:
Character wise Double array Dictionary
Matched 25 candidates
   1 crawdad: ChaRActer Wise Double Array Dictionary (score=2656)
   2 chided: CHaracter wIse DoublE array Dictionary (score=2064)
   3 UNchided: CHaracter wIse DoublE array Dictionary (score=2064)
   4 Scheduled: CHaracter wisE DoUbLE array Dictionary (score=2032)
   5 cheddar: CHaracter wisE Double array DictionARy (score=1862)
```

## Scoring

The candidates are printed in score order.
The scores are assigned based on the following ideas:

- The more forward letters of each word in a description, the higher the score.
- The more letters matched, the higher the score.

More formally, given a text $T$ and a set of positions $\{ i_1, i_2, \dots, i_m \}$ of $T$ such that $T[i_j]$ is not a space,
we define the score of the subsequence $T[i_1] T[i_2] \dots T[i_m]$ as

$$ \sum_{j \in [1,m]} 2^{\ell_{\max} - d(i_j)}, $$

where $\ell_{\max}$ is the maximum length of a word obtained by separating $T$ with a space, and
$d(i)$ is the distance from $T[i]$ to its preceding space (assuming $T[-1]$ is a space).

## Complexity

Enumerating all possible subsequences takes $O(2^n)$ time for an input text of length $n$.
To perform this enumeration in practical time, we generate subsequences on a trie and early prune those that are not candidates.
Furthermore, if the number of candidates exceeds 10k, the process will be forced to terminate.


## TODO

 - Devise more reasonable scoring.
 - Prepare richer word dictionaries.

## Credits

`wordlist/words.txt` is a list of English words provided by [dwyl/english-words](https://github.com/dwyl/english-words).
