//////////////////////////////////////////////
//                                          //
//                  Client                  //
//                                          //
//////////////////////////////////////////////
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

// Localhost with a port in it
const LOCAL_HOST: &str = "127.0.0.1:8080";

// The buffer size of messages
const MESSAGE_SIZE: usize = 32;

fn fn main() 
{
    // Create a mutable client which is a TCP stream 
    // Connect it to our local here ==> IP with the port 
    let mut client = TcpStream::connect(LOCAL_HOST).expect("Failed to connect");
    // We want our client to be non-blocking
    // Set the flag non-blocking to true
    client.set_nonblocking(true).expect("Failed to initiate non-blocking"); 
    // Instantiate channel and assign it to a string type
    // We are going to be sending a bunch of strings through channel
    let (sender, receiver) = mpsc::channel::<String>();

}