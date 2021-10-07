# Dusk
Dusk is a programming language I've been working on on and off for the past while now.
It's still very much in its infancy (... a quick look through this repo will show that) so don't get too excited just yet.
There are a lot of bits and pieces scattered around and I'm still not 100% on some of the project structure. Sit tight!

## Here be dragons!
I'm mid-rewrite at the moment, so currently a couple things aren't present here. Bear with me!

## A quick note
Dusk currently requires Rust 1.53+ due to the use of or-patterns and `IntoIterator` for arrays.
Since 1.53 was recently released, you *should* be able to run everything here on stable now.
Otherwise, you'll need to be on nightly. Sorry!

Since this project uses LLVM, you're gonna need to have the `llvm-12` and `llvm-12-dev` (or equivalent) packages installed.
On Windows, this is quite a bit more complex. See below.

### Advice for the foolish
<details>
<summary>This is mostly for my own good. Click to expand.</summary>

If you're compiling on Windows, you're in for a fun time.
Notably, if you get weird assertion failures while building `llvm-sys`, make sure you have the right toolchain platform.
That is, if you built LLVM with MSVC, `rustup default nightly-msvc` (or similar) is your friend.

Past that, here's what I ran on Windows to get things to function:

``` sh
# This will take a while. Have fun.
git clone https://github.com/llvm/llvm-project
git checkout releases/12.x
cd llvm-project
cd llvm
cmake . -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_ASSERTIONS=ON -G "Visual Studio 16 2019" -Thost=x64
# At this point, `LLVM.sln` will exist. You're gonna wanna find this in Visual Studio, set the build to `Release`, and then build `ALL_BUILD`.
# In truth, we could use `cmake --build .` here. But that will not run anything in parallel and it will be quite slow. So don't do that.
# I'm not sure if this next step is required? But it ended up working for me. So your mileage may vary.
cmake --build . --target install
# After this, decide on where you wanna put LLVM. I chose `C:/llvm-12` because I'm an idiot.
$env:LLVM_SYS_120_PREFIX = "..." # Where "..." is your install location.
# Now copy the contents of `llvm-project/llvm/Release` to your install folder.
# You'll also want to copy `llvm-project/llvm/include` to your install folder - the whole thing, not just the contents.
# Once you're done, running `Get-ChildItem -Name` in your install folder should look like this:
bin
include
lib
libllvm-c.args
libllvm-c.exports
# Before trying to build, it's a good idea to run `cargo clean`. Otherwise, you're all set!
```

If you're on a *nix system you already know what you're doing.
</details>

## What can I do at the moment?
Not much, really. If you clone the repo and run the following:
```sh
cd prompt
cargo run
```
You'll wind up in a small interactive prompt. All this does at the moment is run the parser on a snippet of code and
print the resulting AST (and any errors raised) to stdout, but its capabilities will grow in the future. I'm hoping that
eventually I'll be able to turn it into something similar to `GHCI` or `iex`.

## Where is this going to go?
I have a lot of big ambitions but I decided that it was better to start small for the time being. Later down the track
I'd like Dusk to have a strong type system with variadics and type inference via Hindley-Milner. Unfortunately
implementing Hindley-Milner is a bit above my pay grade at the moment, so I have a lot of reading to do. Until then I'll
probably stick with something less sophisticated.

## What's next?
- [ ] Basic expression/statement parser
- [ ] Basic semantic analysis
- [ ] Symbol resolution and symbolic analysis
- [ ] Type inference
- [ ] Type checking
- [ ] Codegen via LLVM IR

After these are done I'm going to expand the language a bit - implementing a heap allocator will probably be the
next big priority. Then I want to focus on some really basic growable types (namely `String`s) and a small standard library.
