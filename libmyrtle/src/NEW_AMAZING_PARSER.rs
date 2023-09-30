/*
A new parser could be built for myrtle:
A state machine tokenizer takes characters as input and when tokens are recognized,
they're emitted as an enum instance.

The second stage is the symbol mapping: in this phase, symbols are indexed and from now on,
their name is replaced by an integer.

The parser is another state machine that takes indexed as input and builds a program out of it.
*/