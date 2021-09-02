use futures::stream::{self, StreamExt};
use std::iter;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

async fn sieve() {
    let (tx, rx): (UnboundedSender<i32>, UnboundedReceiver<i32>) = unbounded_channel();
    tokio::spawn(async move {
        for i in 2..20000 {
            tx.send(i).ok();
        }
    });
    stream::iter(iter::repeat(()))
        .scan(rx, |rx, _| {
            let (tx2, mut rx2) = unbounded_channel();
            std::mem::swap(rx, &mut rx2);
            async {
                let prime = rx2.recv().await?;
                println!("{}", prime);
                tokio::spawn(async move {
                    while let Some(i) = rx2.recv().await {
                        if i % prime != 0 {
                            tx2.send(i).ok();
                        }
                    }
                });
                Some(())
            }
        })
        .all(|_| std::future::ready(true))
        .await;
}

#[tokio::main]
async fn main() {
    sieve().await;
}