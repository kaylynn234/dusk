# Dusk
Dusk is a programming language I've been working on on and off for the past while now.
It's still very much in its infancy (... a quick look through this repo will show that) so don't get too excited just yet.
There are a lot of bits and pieces scattered around and I'm still not 100% on some of the project structure. Sit tight!

## What can I do at the moment?
Not much, really. If you clone the repo and run the following:
```sh
cd prompt
cargo run
```
You'll wind up in a small interactive prompt. All this does at the moment is run the parser on a snippet of code and
print the resulting AST to stdout, but its capabilities will grow in the future. I'm hoping that eventually I'll be
able to turn it into something similar to `GHCI` or `iex`.

## Where is this going to go?
I have a lot of big ambitions but I decided that it was better to start small for the time being. Later down the track
I'd like Dusk to have a strong type system with variadics and type inference via Hindley-Milner. Unfortunately
implementing Hindley-Milner is a bit above my pay grade at the moment, so I have a lot of reading to do. Until then I'll
probably stick with something less sophisticated.

## What's next?
[x] Basic expression/statement parser
[ ] Basic semantic analysis
[ ] Symbol resolution and symbolic analysis
[ ] Type inference
[ ] Type checking
[ ] Codegen via LLVM IR

After these are done I'm going to expand the language a bit - implementing a heap allocator will probably be the
next big priority. Then I want to focus on some really basic growable types (namely `String`s) and a small standard library.
