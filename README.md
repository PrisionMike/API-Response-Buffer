# Edge API Cache - No more a learning project

Technially it still is but the contribution to it will be more regular and the end delivery will focus on a profesional grade finished product.

## What is it again?

A cache for Web API responses. Like a tank (Queue) to pre-fetch the responses so this does not bottle your code.

**_ I STILL DON'T KNOW IF THERE IS AN INDUSTRIAL APPLICATION FOR IT _**

Everyone who I shared this idea with found it cool and worthy of a product. I had a niche application for it and will help me get a nice handle of a proper system/project development in Rust.

## Target build: WASM

### Why?

I want to actually run and feel the Build-Once-Run-Everywhere dream. With the obvious merits of this then becoming platform independent and easy to deploy. You can be working with an edge system where the remote system's specifiication is not known to you. You can simply deploy this system there and fetch the responses.

WASM also has good bindings to simply use in the programming language of your choice.

Given the speed, and availability with other other languages, **WASMER** was opted as the target WASM runtime.

### How?

The whole "Supply System" will be a portable WASM module. It will have _imports_ which will accept the API request string and related meta data for the tank. This will let it expose its _exports_ to the user which will be the "local APIs" user should use.

## Roadmap.

I don't know how SW versioning works. Also, I want to plugin `tokio` somewhere in here but I can't even imagine where it will lie in my project.

- **v0.1.0** A command line binary that can work as a tank of API responses.

  - ~~A `Dispenser` is created with a given web API and capacity. It has a tank in which it loads the said number of responses in it.~~
  - ~~It has a _tap_ to dispense the stored cache.~~
  - It has a _button_ which can be pressed to refill the tank.
  - Encapsulate the responses for better UX.
  - Make sure the water comes in the tank. There are no error bubbles in the supply. (Errors due to timeout are handled.)
  - Switch to _auto-refill_ the tank after a certain time.
  - _Auto-refill_ now works based on the level of the water in the tank.

- Make the async await flow sensible.

- Avoid the motor to burnout if there's no supply.

- Error Handling. Testing? Fuzzing?

- Burn it as a WASM module.

- Make it ready for remote system deployment. (Do the Docs)

- Security updates.

- Privacy updates.

- Install Purifiers. (Post process)

- (Make the code Async)?

- Multi API support.

- Multi threading for faster fetching.

- Multi User support.

.
.
.

- You specify a full schema of all APIs you want to cache. How many fetchers for it, how many users for it, what errors it can throw, what to do in such errors. Purifier functions, and what not. All of this, written in an in-house schema, or in a standard json/yaml/xml file. The user makes that file. The module will receive a single input - that file - and handle it all.

(Perhaps an encoding scheme to define all this instead of a structured text)

## Miscellaneous

- _GWS_ stands for _Ganga Water Supply_. The water supply system at my home.
