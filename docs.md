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