use std::thread;
use std::sync::mpsc;

fn generate(tx:   std::sync::mpsc::Sender<i64>){
    let mut i=2;
    loop{
        // let mut val =i;
        // val.push_str(&i.to_string());
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

        if prime>2000{
            break;
        }
    }
}