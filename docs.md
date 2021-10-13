# Main Program (Server starter)

Runs via commandline by simply entering the command: `Jhaadi -k 'peechhe'`  
Here **Jhaadi** is the command and **-k** flag is used to send the argument *'peechhe'*  
The Program should return (along with other log mentions optionally):  
```
Server running at http:/localhost:9623/choli/
```

If the said address is fed to a browser, the screen should simply read:
```
peechhe
```

This server starter launches the server by chosing an arbitrary port.  

# Server
Upon receiving any GET request matching '/',
The server should `stdout` the **-k** argument received by the user (*peechhe* in this case).

## Procedure

Upon parsing the CL input and validating them, a server is start. a simple blank server.  
The *Main Program* will then make a POST request to the server with the valid input received in the CLI.  
The server, upon receiving such a POST request will validate the input, make a separate resource
dedicated to the input, and notify the main program, that such a resource has been generated successfully.  
The server will then return the full address of the resource. This is the point the user will ping.

### Figuring out the implementation - baby steps

 - Make a destination (route) function for any POST request received at '/'.
 - Use an app like Postman to send a POST request to it and see if it lands at the right destination.
 - Try sending something in the body as well and see if you can display it on the screen.
 - Make a separate function in the `main.rs` file for sending the POST request. The fucntion will probably be tokio async and will accept the input string to be sent as an argument.
 - Separate function trick didn't work somehow.
 ```
 Err(reqwest::Error { kind: Request, url: Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(8000), path: "/", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 10061, kind: ConnectionRefused, message: "No connection could be made because the 
target machine actively refused it." })) })
```
- Let's try having a different file to post the request and see if that works. Later we'll figure out how to launch the server stably and passing this ready command internally. Good night.