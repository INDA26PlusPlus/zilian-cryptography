use core::time;
use std::{env, io, path, thread::sleep};

use crate::network::Network;
mod network;
mod file_manager;
mod encryption;
mod decrypting;

fn main() {
    // Main program that runs on start

    // Pause time for program (not needed at all btw, just gonna keep this here for the time being)
    let one_sec = time::Duration::from_secs(1);

    // Establishes connection between host and server
    // Could make two separate programs for host and for client and run them separately, but I'll just do this.

    let connection = user_interface();

    if connection.is_some() {

        let mut connection_type = connection.unwrap();
        // If current user is host
        if connection_type.is_host() {

            // Loop for receiving from client
            loop {
                // Loops and prints received message from client.
                match connection_type.receive_message() {
                    Ok(Some(message)) => println!("{}", message),
                    Ok(None) => println!("Nothing Yet"),
                    Err(e) => println!("{:?}", e)
                }
                sleep(one_sec);
            }
        }
        else {
            // Loop for sending to host
            loop {
                // Creates a message to send across the network
                let mut choice = String::new();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Invalid input.");

                // Take choice send it across network to host
                connection_type.send_message(&choice);
            }
        }
        

    }
    else {
        println!("Goodbye!")
    }
}

pub fn user_interface() -> Option<Network> {

    // HERE WE DETERMINE WHAT TYPE OF GAME THIS IS
    // THAT IS ONE OF:
    // host, client, local (none)
    // Then we setup the network depending on that.

    println!("What do you want to be?");
    println!("1. Host");
    println!("2. Client");
    println!("3. Local/Test");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Invalid input.");
    
    let network = match choice.trim() {
        // Setup local as host
        "1" => {
            println!("Searching for client to join on PORT: {}...", network::PORT);

            match network::Network::establish_as_host() {
                Ok(network) => return Some(network),
                Err(error) => {
                    println!("Search failed: {:?}", error);
                    return user_interface();
                }
            }
        }
        // Setup local as client
        "2" => {
            println!("Enter address to join (leave empty for 127.0.0.1:6767):");

            let mut addr = String::new();
            io::stdin().read_line(&mut addr).expect("Invalid input.");
            if addr.trim().is_empty() {
                 addr = "127.0.0.1:6767".to_string();
            } 
            else { 
                addr = addr.trim().to_string();
            };

            match network::Network::establish_as_client(&addr) {
                Ok(network) => return Some(network),
                Err(error) => {
                    println!("Failed to join {}: {:?}", addr, error);
                    return user_interface();
                }
            }
        }
        // Setup local as "local"
        _ => return None,
    };
}

pub fn transfer_loop() {
    todo!()
}

