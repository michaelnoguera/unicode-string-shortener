# Unicode String Shortener

The idea of this project is to compress text in terms of bytes or characters used while maintaining its human readability. Using this program you can enter more text than intended into a limited-size form field.

For the intents of this project, anything that looks close enough to a latin letter to be readable is considered acceptable, even if it may have an entirely different meaning.

Here is an example of program functionality.
```
Enter string to shorten: aether
Input:                         aether               (6)
Shortest in bytes:             æther                (6)
Shortest in characters used:   æᵺer                 (4)
```

### Structure

A human-readable list of shortenings used is available in `map.tsv`. The columns in that file are: unicode codepoint, unicode character, ascii equivalent strings. There can be more than one ascii equivalent string, columns are added for each additional one. However, no two unicode characters can be translated to the same string (the program checks for this and will error).

To update the computer-readable list in `map.bincode`, delete the `map.bincode` file and run the program. A new bincode will be produced from `map.tsv`.

### Usage

This program is unfinished and has no nice interface yet. It has no obscure dependencies not in cargo, so 