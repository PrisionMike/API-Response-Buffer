## Main Program.

Upon running the program with the sample code given in *notes.md*, the program should print the given number of responses of the sent. API.  
Since the output of most API responses would be JSON formatted text, add another flag that would identify the *data* field of the JSON reponse. This would be an optional flag. If provided, the identifier would be used to parse the JSON response, pull it out and queue it up. All the data from the responses will be concatenated in the right order to create a master array that will be printed.