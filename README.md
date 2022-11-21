# Summer of Rust Lab 4: The Comic Book Shoppe 2

In this lab, we're going to practice using enums and pattern matching. We'll be
working on top of what we did in the last lab, but the code from those sections
will be already implemented for you.

To try and be more clear, I changed the headings of different sections to
include a few different tags:

- (info): This section teaches something
- (impl): This section wants you to write some code
- (next): Something to try after the lab

## The Comic Book Shoppe II

The Comic Book Shoppe 2 is a second Comic Book Shoppe in downtown Ottawa.
However, as of writing, it's currently closed. That means it's the perfect time
to do some experimental tests for the R.U.S.T. (Realtime and Updated Shop
Tender) system! 🎲 In this lab, we're going to set up the R.U.S.T. system to
allow the stores to compete against each other to find who has the best set of
cards.

### (info) Running it

For this lab, when you run the `main.rs` file, it will print out battles between
cards, and a battle between two stores. Not everything is implemented for this
yet, but once you fix it up, you should be able to see a full fight when you
`cargo run`!

Also note, the tests for this lab are more strict than last time. `cargo check`
gives you some linting recommendations, but `cargo clippy` gives far more. To
make sure that you have `clippy`, you can run this in your terminal:

```bash
rustup component add clippy
```

Make sure that `cargo clippy` doesn't show any warnings before you commit your
code!

### (info) Module time

The code for this lab is split up into several modules. We have the `main.rs`
file, and it references `card.rs` and `shop.rs`. Tests are now stored in
`test.rs`. This allows us to start splitting up the code a bit more nicely.

You'll also notice that we now have to add a keyword to our structs:

![](https://cdn.discordapp.com/attachments/968184771102507031/975843006655627294/lab-4-1.png)

We've also had to make the methods of each struct public, as well as enums. When
might we not want things to be public?

- A method might not be public if it should only be called from within another
  method on the struct
- A field might not be public if you don't want people using their own code to
  access it
- A struct might not be public if it's only going to be used within an API and
  not by the end-user

### (impl) The fight result enum

First, we'll need to define an enum to represent the result of a fight. In our
case, there are four outcomes of a fight: win, loss, draw, and tie. These can
each be nicely represented as an enum variant! The enum definition for
`FightResult` is at the top of `main.rs`, so go there to change it. The reason
that it's in this file is that it will be used by both the `Shop` and the `Card`
structs.

### (impl) The fight method

Next, we're going to set up fights between `Card`s. This will be a method called
`fight` on the `Card` struct. It will take in an instance of a card with
`&self`, and another instance as `other`. Here are the rules of a fight:

- If both cards deal enough damage to kill one another, return a tie
- If this card deals enough damage to kill the other card, return a win
- If the other card deals enough damage to kill this card, return a loss
- If neither card deals enough damage to kill the other, return a draw

Depending on the outcome of the fight, this method should return a
`FightResult`. There are two (trivial) ways to implement this method:

- If statements
- Match statement

The match statement syntax is a bit more complex, but it's far more elegant. For
this lab, we'll implement the method with a match statement. [Rust by Example
has a great
chapter](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) on
how to work with match statements.

Below are some hints to get the match function working. Feel free to use them if
you get stuck, but try on your own first!

<details>
<summary>Hint 1</summary>
First, we need something to match on. A tuple would be nice; we could store both states in it. The first state is whether this card deals enough damage to kill the other card, and the second is whether the other card deals enough damage to kill this card. Check out hint 2 if you want to see what that would look like, or give it a try on your own!
</details>

<details>
<summary>Hint 2</summary>
<img src="https://cdn.discordapp.com/attachments/968184771102507031/975829846112493658/match.png">
We can make a tuple that stores booleans to represent the two states. Next, we need to match on that tuple. There are four possible outcomes from two booleans:

- (true, true)
- (true, false)
- (false, true)
- (false, false)

See if you can work these into the match statement! In hint 3, we can see a full
breakdown.
</details>

<details>
<summary>Hint 3</summary>
<img src="https://cdn.discordapp.com/attachments/968184771102507031/975850975371866152/lab-4-2.png">
</details>

### (info) Impl Display

You'll notice that in the `card.rs` file, there is a `Display` impl for `Card`.
This is because we want to print out the card in a certain way that we want to
specify. Another thing to note is that this impl looks a little different than
our normal ones. This is because we're implementing a "trait" on the struct,
specifically the `Display` trait. We'll be learning how to make our own traits
in a few weeks!

### (impl) Shop battle

Now that cards can fight, it's time for shops to be able to fight! In a shop
battle, each card fight from one shop fights against every card in the other
shop. The shop that wins the most fights wins the battle. If both shops win the
same number of battles, then the battle is a tie.

For this, we'll use the same `FightResult` enum as we did with the `Card`
method. You'll need to work on the `fight_store` method on the `Shop` struct.

Remember, when you're implementing this, you can use the `print_fight` method
instead of `fight` if you want to see a log of the fight. Feel free to change
this print method if you like!

### (info) A bit on iterators

To simplify some of the code from last week, we've changed the implementation of
the Shop methods to use iterators rather than loops. Although we haven't learned
about iterators just yet, we can still break down what's going on.

```rust
fn most_expensive(&self) -> u32 {
  self.cards
    .iter()                 // Convert `Vec<Card>` to `Iterator<Card>`
    .map(|card| card.price) // "map" each input element to an output
    .max()                  // get the "max" element in the iterator
    .unwrap()               // unwrap the result
}
```

A few things to note here:

- The `map` method is doing most of the work here. It's taking each element of
  the iterator, and applying a function to it. In this case, we're mapping each
  element to the `price` field of the card.
- In the `map` function, we didn't need to use the `return` keyword. Clean!
- We have something called `unwrap` happening at the end. This is important for
  error handling; if our iterator is empty, the program will panic here. In a
  real-world case, we'd handle it a bit differently. We'll find out more about
  this when we learn about `Option`s!
- In this example, each function is on a new line. Rust has "function chaining",
  which allows us to make a pipeline for our data to go through. Normally, short
  iterator chains like this would get formatted to be on the same line.
  
Also, don't be worried if this doesn't seem like a natural way to do it! The
intention here is to expose you to the idea of iterators and to see a bit about
how they work. In future sessions, we'll play with them a lot more. But if you
do learn them ahead of time, feel free to use them in your solutions!

### (next) Rustlings

This week, we should be far enough to do the Rustlings from move semantics to
modules. The move semantics Rustlings are certainly a brain teaser as it uses
the concepts of ownership and borrowing. Always feel free to ask questions in
#rust-questions when you run into any problems!

### That's all!

See you next week 🏖️

## License

The Summer of Rust Labs is duel-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))