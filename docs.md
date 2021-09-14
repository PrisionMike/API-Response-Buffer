# Main Program (Server starter)

Runs via commandline by simply entering the command: `Jhaadi -k 'peechhe'`  
Here **Jhaadi** is the command and **-k** flag is used to send the argument *'peechhe'*  
The Program should return (along with other log mentions optionally):  
```
Server running at http:/localhost:9623/
```

If the said address is fed to a browser, the screen should simply read:
```
peechhe
```

This server starter launches the server by chosing an arbitrary port.  

# Server
Upon receiving any GET request matching '/',
The server should `stdout` the **-k** argument received by the user (*peechhe* in this case).