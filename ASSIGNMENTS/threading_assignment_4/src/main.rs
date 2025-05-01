use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];
    let termination_tx = tx.clone();

    // TODO: Create 2 producer threads
    let items_per_producer = ITEM_COUNT / 2;
    for id in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(id, tx_clone, items_per_producer);
        });
        handles.push(handle);
    }

    
    // TODO: Create 3 consumer threads
    for id in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        handles.push(handle);
    }

    drop(tx);

    // After producers are done, send termination signals
    thread::sleep(Duration::from_secs(2)); // Give time for producers to finish
    for _ in 0..3 {
 
        termination_tx.send(TERMINATION_SIGNAL).unwrap(); // workaround for dropping tx

    }
    // TODO: Wait for all threads to finish
    
    println!("Main thread waiting for all threads to finish...");

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    for i in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} produced value {} (item {}/{})", id, num, i + 1, item_count);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished producing.", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let msg = rx.lock().unwrap().recv().unwrap();
        if msg == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal. Exiting.", id);
            break;
        } else {
            println!("Consumer {} received value: {}", id, msg);
            thread::sleep(Duration::from_millis(150));
        }
    }
}