use std::thread;
use futures_lite::future;
fn generate(tx:   async_channel::Sender<i64>){
    let mut i=2;
    
    loop{
        future::block_on( tx.send(i)).unwrap();
        i+=1;
    }
}

fn filter(src : async_channel::Receiver<i64>, dst:async_channel::Sender<i64>, prime: i64) {
    loop{
        let i=future::block_on(src.recv()).unwrap();
        if i%prime!=0{
            future::block_on(dst.send(i)).unwrap();
        }
    }
}


fn main() {
    let (tx, mut rx) = async_channel::bounded(1);
    thread::spawn(move || generate(tx));
    loop{
        let prime =future::block_on(rx.recv()).unwrap();
	    //println!("{:?}",prime);
        let (tx2,rx2) = async_channel::bounded(30);
        thread::spawn(move || filter(rx,tx2,prime));
        rx=rx2;
        if prime>20000{
            break;
        }
    }
}