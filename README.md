# Chat App 
🏁🔮  The goal of this project is writing a simple  Chat Application (server-client) in Rust.
## Dependencies


### macOS
This project needs Rust compiler (rustc) and the Rust package manager (cargo).

### Building on macOS
#### Rust Installation :
 For installation it's advised to use rustup.
 
  	 $ brew install rustup

Use rustup to install the Rust compiler and the Rust package manager.
	
    $ rustup-init


1. Clone the repository.
2. Open your Terminal
3. Change the directory to server.
4. Run  <code>$ cargo run</code> 
5. Change the directory to client.
6. Run  <code>$ cargo run</code> 
7. Write your messages !


📌📌📌  If you try to send a message that's longer than 32 bytes it doesn't actually break anything. It just cuts off the message at 32 bytes.



![](Gifs/Chat.gif)