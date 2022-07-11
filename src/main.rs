
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt}, 
    net::TcpListener, sync::broadcast
};

const BUFFSIZE : usize = 16384;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    let (_tx, _rx) = broadcast::channel::<[u8; BUFFSIZE]>(10);

    loop {  
        let (mut socket , _addr ) = listener.accept().await.unwrap();
        let tx = _tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (mut read_sock, mut write_sock) = socket.split();
            // let mut reader = BufReader::new(read_sock);
            // let mut line = String::new();
            let mut buffer = [0u8; BUFFSIZE];
        
            loop {
                tokio::select! {
                    result = read_sock.read(&mut buffer) => {
                        let size = result.unwrap();
                        if size == 0 { 
                            break;
                        }
                        tx.send(buffer).unwrap();
                    },
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        write_sock.write_all(&msg).await.unwrap();   
                    }
                }
            }
        });
    }
}
