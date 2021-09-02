use std::thread;
// use futures_lite::future;
use crossbeam_channel::{bounded, Sender,Receiver};
// use crossbeam_utils::thread;
fn generate(tx:   Sender<i64>){
    let mut i=2;
    
    loop{
        tx.send(i).unwrap();
        i+=1;
    }
}

fn filter(src : Receiver<i64>, dst:Sender<i64>, prime: i64) {
    loop{
        let i=src.recv().unwrap();
        if i%prime!=0{
           dst.send(i).unwrap();
        }
    }
}


fn main() {
    let (tx, mut rx) = bounded(1);
    thread::spawn(move || generate(tx));
    loop{
        let prime =rx.recv().unwrap();
	   //println!("{:?}",prime);
        let (tx2,rx2) = bounded(30);
       let _= thread::spawn(move || filter(rx,tx2,prime));

        rx=rx2;
        if prime>20000{
            break;
        }
    }
}