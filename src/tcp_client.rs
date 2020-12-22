use std::io::{self,BufRead,BufReader,Write};
use std::net::TcpStream;
use std::str;

/* rake connection directed IP address and Port */
pub fn connect (address: &str) -> Result<(), failure::Error>{
    let mut stream = TcpStream::connect(address)?;
    loop{
        /* send input data from socket*/
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;// encode byte stream to appropriate format
        /* display received data from socket */
        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        print!("{}",str::from_utf8(&buffer)?);
    }
}






