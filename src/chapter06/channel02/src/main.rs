use std::thread;
use std::sync::mpsc;
// use std::time::Duration;
fn generate(tx:   std::sync::mpsc::Sender<i64>){
    let mut i=2;
    loop{
        tx.send(i).unwrap();
        i+=1;
    }
}

fn filter(src : std::sync::mpsc::Receiver<i64>, dst:std::sync::mpsc::Sender<i64>, prime: i64) {
    loop{
        let i=src.recv().unwrap();
        if i%prime!=0{
            dst.send(i).unwrap();
        }
        //thread::sleep(Duration::from_millis(1));
    }
}


fn main() {
    
    let (tx, mut rx) = mpsc::channel();
    thread::spawn(move || generate(tx));
    loop{
        let prime =rx.recv().unwrap();
	   println!("{}",prime);
        let (tx2,rx2) = mpsc::channel();
        thread::spawn(move || filter(rx,tx2,prime));
        rx=rx2;
        //thread::sleep(Duration::from_millis(1));
        if prime>20000{
            break;
        }
    }
}