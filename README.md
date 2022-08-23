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

I don't know how SW versioning works

- **v0.1.0** A command line binary that can work as a tank of API responses.

- Burn it as a WASM module.

- Make it ready for remote system deployment. (Do the Docs)

- Security updates.

- Multi API support.
