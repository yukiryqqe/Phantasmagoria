use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt,AsyncWriteExt};

#[tokio::main]
async fn main() {
    let addr="127.0.0.1:6666";
    let listener=TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Listening on {}",addr);
    loop {
        match listener.accept().await {
            Ok((socket,addr))=>{
                println!("New client:{:?}",addr);
                tokio::spawn(async move {
                    if let Err(e)=handle_client(socket).await {
                        eprintln!("Failed to handle client: {}",e);
                    }
                });
            },
            Err(e)=>println!("Couldn't accept connection {:?}",e),
        }
    }
}

async fn handle_client(mut stream: TcpStream)-> std::io::Result<()>{
    let mut buffer =[0;1024];

    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(n) if n==0 => return Ok(()),
            Ok(n)=> n,
            Err(e)=> {
                eprintln!("Failed to read from socket {}",e);
                return Err(e);
            }
        };
        if let Err(e)=stream.write_all(&buffer[0..n]).await {
            eprintln!("Failed to write to socket: {}",e);
            return Err(e);
        }

    }
}