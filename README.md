# The Rust Programming Language

This is a repository of the exercises found in the official [Rust book](https://doc.rust-lang.org/book) that I've completed myself often with contextuized inline comments provided given my understanding of a topic or concept.

## primary takeaways

- memory allocation : stack vs heap. stack for fixed size, and heap for dynamic (unknown)
- ownership can be challenging : references
- enums are nice and we can add behavior to them with methods
- pattern-matching and the `match` control flow operator looks like it's borrowed from ML, but it's a familiar construct for me having programmed in Elixir
- strings : Rust doesn't hide the complexity that it truly is and seems to provide some performance benefits in particular types of applications I've yet to work with in Rust
- error handling : very nice and forces the programmer to think about failure modes, and I'm already seeing myself frown upon the liberal use of `unwrap()`
- generics : it's one of the first languages I've worked with that has them, and having done some Go, I it is now acutely apparent to me why that community has been split about it. I come from mostly a dynamic programming language background, so it's hard for me not to like generics
- lifetimes are still a fuzzy thing for me, but I do know that they are tied specifically to references
- tests : has borrowed things from Python like doctests and control over what sets of tests are run
- enjoyed implementing the minigrep project that draws some inspiration from the insanely fast grep substituted, ripgrep by BurntSushi that I love using
- absolutely love the functional language features it has that are "zero cost abstractions" that incur no performance penalty. It's more of a style thing, but it's much appreciated because I like more expressive code often written declaratively. I didn't like Go because of it's lack of this, but Go by contrast is very very simple and stays close to its C roots
- smart pointers : found in various other languages as well, but the memory safety guarantees and promises seem nice
- fearless concurrency : thread safety primitives like mutex and atomic reference counter are sparking some ideas and excites me. I've familiarity with deadlocks and often illustrated in a problem known as, "The Dining Philosophers", but am intrigued at possibly using Rust to illustrate and mitigate
- Rust to me is not really an object-oriented programming language, but parallels can be drawn and it makes it a point to distinguish their trait objects from traditional objects -- that is to say that in most OOP languages, the data and behavior are combined whereby Rust has clear separation between them similarly to functional programming languages. I come away from this thinking we can just call this "trait-based programming" and I'm really liking this as the default instead of me having to discuss mixins and why not to use inheritance and base classes as a dumping ground for shared behavior (methods)
- trait objects with "dynamic dispatch" can provide flexibility to aid maintainability with the tradeoff being runtime performance
