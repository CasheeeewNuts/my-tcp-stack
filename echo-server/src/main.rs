use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, thread};
use std::str::{from_utf8};

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];

    run_server(addr)?;

    Ok(())
}

fn run_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;

    loop {
        let (mut stream, _) = listener.accept()?;

        thread::spawn(move || {
            let mut buf = [0u8; 1024];

            loop {
                let bytes = stream.read(&mut buf).unwrap();

                if  bytes == 0 {
                    return;
                }

                print!("{}", from_utf8(&buf[..bytes]).unwrap());

                stream.write_all(&buf[..bytes]).unwrap();
            }
        });
    }
}
