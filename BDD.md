<!-- # Here's To Hoping -->
<!-- More info required on how local APIs are made and how they work -->
# Behaviour Driven Development

You write the documents first and code later.
Web based APIs are slow. One often wants to have lot of responses
cached in a buffer to enhance the response speed.
What we do is create a local API which simply gets the prefetched
values as and when requested.

# End to End Description

### INPUT:  
    tanki-chala-do {api-request} [Number of responses to store]  

### OUTPUT:  
    __________________*************___________________  
    Local API created successfully!\
    New API name:   amirkhan\
    Fetch command:  amrirkhan.get_response(N)  
    Fetch output:   O1,O2,O3
    Status check command:
                    amirkhan.get_status
    Status check response:
                    Available responses in cache: {N}
                    Last Request at: {Date - Time}
    Cache level:    amirkhan.get_lev_check  (Available responses in Cache)
    Stale check command:
                    amirkhan.stalecheck (Last Request at)
    Flush command:  amirkhan.flush
    Refill command: amirkhan.refill(N)
    ________________**************___________________

I guess I should now define the different modules the program will have
and how each of the will *behave* 😋

# Modules

## M1: Initiator

The Inlet is powered on using 2 arguments: **the API request text** and the
**number of responses = N** to cache.
Upon being turned on, it creates a **Tank** of size *N*. This tank is a queue.

It runs as a background process for the loading and maintenance of the *Tanks*.

When the *Initiator* receives a `Ready` signal from the tank, it needs to create a **Tap**.

The **Tap** will be instantiated by the **Tank** instance and 2 available port numbers
for sending the request and receiving the response respectively.

## M2: Tank

FIFO buffer. It is filled with *N* `None`.

### Recharge process
The program then makes *N* API requests and pushes the response of each request
to the Q-tank as `Some(Response)`. [ loop (request and push) N times]
Upon the successfull filling of the tank, the tank will save the current date and time with itself by the name
`Last Response Time Stamp`. After that, the *Tank* will signal the initiator that it is `ready` to be
fitted with a tap (Converted into a local API)

It can also receive a `Recharge` command, upon which it initiates the *Recharge process*.  

On receiving a `Flush` command, the queue will be cleared. (Filled with `None`)  

On receiving a `Level Check` request, it will return the number of currently available responses.  

On receiving a `get Capacity` command, it will return its capacity.  

On receiving a `Stale check` command, it will return the `Last Response Time Stamp`.  

## M2: The fetch-from-tank API (Tap) [Local API]

A Tap is provided to a **Tank** for retrieval of the stored data along with 2 port addresses
to receive response and send response requests.

When a user asks the Tap to draw *M* amount of data from the tap, the tap
will create a *Glass* of capacity *M* and place it underneath it.
The tap will then pull *M* amount of data sequentially from the tank.
The tap will stop filling the glass after either:  

1) *M* responses are pushed onto the tank.  
2) A `None` is pushed in the tank.  

The tap will then check the tank level.  
If the level of the tank is less than 1/3rd of its capacity, the
tap will request a refill.

## M3: Outpour

Given a *Glass* from the Tap, it will format the content of the glass
using the delimiter (comma+CRLF), and send it to `stdout`.