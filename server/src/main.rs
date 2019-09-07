//////////////////////////////////////////////
//                                          //
//                  Server                  //
//                                          //
//////////////////////////////////////////////

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

// Localhost with a port in it
const LOCAL_HOST: &str = "127.0.0.1:8080";

// The buffer size of messages
const MESSAGE_SIZE: usize = 32;


fn main()
{
    // Instantiate server 
    let listener = TcpListener::bind(LOCAL_HOST).expect("Could not bind socket");
    // Push listener in non-blocking mode
    listener.set_nonblocking(true).expect("Failed to initialize non-blocking");

    // Create mutable vector for clients
    let mut clients = vec![];

    // Instantiate channel and assign it to a string type
    // We are going to be sending a bunch of strings through channel
    let (sender, receiver) = mpsc::channel::<String>();
     
    loop 
    {   
        // Destruct result from listener.accept()  
        // listener.accept() allows us to accept connections to this server
        // socket: TCP stream 
        // address: socket address 
        if let Ok((mut socket, address)) = listener.accept()
            {
                println!("Client {}: CONNECTED", address);
                // Clone sender
                // The socket tries to clone it and then push it to clients vector 
                // We're cloning the socket to push it into our thread 
                let sender = sender.clone();
                clients.push(socket.try_clone().expect("Failed to clone client"));
            }

    }    
                
}

