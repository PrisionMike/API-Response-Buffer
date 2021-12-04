## Sample input:
```
cargo run -- -n 10 -i "https://qrng.anu.edu.au/API/jsonI.php?length=10&type=uint8"

cargo run -- -n 10 -i "https://qrng.anu.edu.au/API/jsonI.php?length=10&type=uint8" -d "data" -t "usize"
```
## Sample output of the API at hand:
```
I => https://qrng.anu.edu.au/API/jsonI.php?length=3&type=uint8

O => {"type":"uint8","length":3,"data":[53,184,52],"success":true}

I => https://qrng.anu.edu.au/API/jsonI.php?length=3&type=hex16&size=6

O => {"type":"string","length":3,"size":6,"data":["2f793a9eed21","dbceb801f33d","859633f81c4a"],"success":true}
```
Having tried *Trait objects* and miserably failing as it required **AT LEAST** manually implementing the `Deserialize` trait to it, and then some.  

The process seems surprisingly complicated. It's a simple case of not knowing what 'type' of data the response would contain. I am now trying to make a struct using a generic, implement/`Derive` the required traits, and hope that all this works.  

It seems you cannot even implement the `Deserialize` Trait for a Trait object (yet?). So I gotta figure out how to do it using generics only. Not Trait Bound.

- Refactoring first, Reassigning scope later - (@PrisonMike, 23/11)