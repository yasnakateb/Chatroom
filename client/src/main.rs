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