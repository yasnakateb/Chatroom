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



