<!-- # Here's To Hoping -->
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
