# SMART_server

## Get the server up and running

Make sure of the following

- You have docker installed and the daemon is running
- You can write docker commands as a non-root user
- You have an IDE that supports devcontainers (ex: vscode)



Clone the repository : 

```sh
 git clone https://github.com/clemgi0/SMART_server.git
 ```

Open the repository in you IDE and then reopen it in a container
(in vscode ctrl+shift+p "Reopen in Container")

In your IDE's integrated terminal type :  

```sh
diesel run migration
```

Launch the server : 
```sh
cargo run
```

## API documentation and testing

This repository contains a postman collection json file that's meant for
documenting the API and for giving examples of its use.

In order to open the collection, (assuming you are running vscode) :  
Install the Postman extention, then authenticate with your postman account and finally import the collection.