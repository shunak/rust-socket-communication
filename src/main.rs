use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;


fn main() {
    env::set_var("RUST_LOG","debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() !=4 {
        error!("Please specify [tcp | udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];

    match protocol{
        "tcp"=> match role {
            "server" => {
                // call tcp server
                tcp_server::serve(address).unwrap_or_else(|e|error!("{}",e));
            }
            "client" =>{
                // call tcp client
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}",e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
           "server" => {
               // call udp server
               udp_server::serve(address).unwrap_or_else(|e| error!("{}",e));
           }
           "client" => {
               //call udp client
               udp_client::communicate(address).unwrap_or_else(|e| error!("{}",e));
           }
           _ => {
               missing_role();
           }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

/* error generate function when a 2nd argument is invalid value*/
fn missing_role(){
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}





