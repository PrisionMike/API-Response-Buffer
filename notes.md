## Sample input:
```
cargo run -- -n 1000 -i "https://qrng.anu.edu.au/API/jsonI.php?length=10&type=uint8"
```
Having tried *Trait objects* and miserably failing as it required **AT LEAST** manually implementing the `Deserialize` trait to it, and then some.  

The process seems surprisingly complicated. It's a simple case of not knowing what 'type' of data the response would contain. I am now trying to make a struct using a generic, implement/`Derive` the required traits, and hope that all this works.  

It seems you cannot even implement the `Deserialize` Trait for a Trait object (yet?). So I gotta figure out how to do it using generics only. Not Trait Bound.

- Refactoring first, Reassigning scope later - (@PrisonMike, 23/11)