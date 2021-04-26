use std::thread;
// use std::sync::mpsc;
// use crossbeam_channel::bounded;
// use std::time::Duration;
fn generate(tx:   async_channel::Sender<i64>){
    let mut i=2;
    
    loop{
        tx.send(i);
        i+=1;
    }
}

fn filter(src : async_channel::Receiver<i64>, dst:async_channel::Sender<i64>, prime: i64) {
    loop{
        let i=src.recv().await;
        if i%prime!=0{
            dst.send(i);
        }
    }
}


fn main() {
    let (tx, mut rx) = async_channel::unbounded();
    thread::spawn(move || generate(tx));
    loop{
        let prime =rx.recv();
	    println!("{:?}",prime);
        let (tx2,rx2) = async_channel::unbounded();
        thread::spawn(move || filter(rx,tx2,prime));
        rx=rx2;
        if prime>20000{
            break;
        }
    }
}