use std::thread;
use std::sync::mpsc;

fn sub_thread(tx:   std::sync::mpsc::Sender<String>){
    let mut i=0;
    loop{
        let mut val = String::from("hi--");
        val.push_str(&i.to_string());
        tx.send(val).unwrap();
        i+=1;
    }
}
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || sub_thread(tx));
    loop{
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
}