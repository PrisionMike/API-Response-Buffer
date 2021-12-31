## Expected local api:


```
127.0.0.1:23541/home ✔️

127.0.0.1:23541/?n=4&flag=true  ✔️

127.0.0.1:23541/sofresh ✔️

127.0.0.1:23541/sohigh ✔️

127.0.0.1:23541/refill ✔️

127.0.0.1:23541/refill?x=2 // x = number of workers/threads/hoses to fill the tank ✔️


```

## Sample input:
```
cargo run -- -n 5 -i "https://qrng.anu.edu.au/API/jsonI.php?length=1&type=uint8"

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

```
Actix allows for services and routes to be added. They are the same thing only services uses neat macros like *rocket.rs* did.
I'll be using routes here as the name will be passed by the main program or preferrably by the user. Thus the compiler may need to know it.
That's a speculation. I have not tried contradicting it.
```