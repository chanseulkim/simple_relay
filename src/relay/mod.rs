
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, 
    net::{TcpListener, TcpStream}, sync::broadcast::{self, Sender, Receiver}
};

const BUFFSIZE : usize = 16384;

async fn client_handle(mut socket : TcpStream, tx : Sender<Vec<u8>>, mut rx : Receiver<Vec<u8>>) {
    let (mut read_sock, mut write_sock) = socket.split();
    let mut buffer = [0u8; BUFFSIZE];
    loop {
        tokio::select! {
            result = read_sock.read(&mut buffer) => {
                let size = result.unwrap();
                if size == 0 {
                    println!("no data");
                    break;
                }
                tx.send(buffer.to_vec()).unwrap();
            },
            result = rx.recv() => {
                let mut msg = result.unwrap();
                write_sock.write_all(msg.as_mut_slice()).await.unwrap();   
            }
        }
    }
}

pub async fn run() {
    let addr = "127.0.0.1:50800";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("server run {}", addr);

    
    let (_tx, _rx) = broadcast::channel::<Vec<u8>>(10);

    loop {  
        let (socket , _addr ) = listener.accept().await.unwrap();
        let tx = _tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            let ip = socket.peer_addr().unwrap().ip().to_string();
            println!("on client {}", &ip);
            client_handle(socket, tx, rx).await;
            println!("out client {}", &ip);
        });
    }
}
