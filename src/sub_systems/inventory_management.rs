use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

use super::receiver;

#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: String,
}

fn receive_goods(sender: Sender<Item>, duration: Duration){
    let mut item_id = 1000;
    println!("Loading process started");
    for _ in 0..(duration.as_secs() * 2) { // Simulate receiving for twice the duration
        let item_name = format!("Product {}", item_id);
        let item = Item { id: item_id, name: item_name};
        sender.send(item).unwrap();
        item_id += 1;
        thread::sleep(Duration::from_millis(500)); // Simulate some delay between items
    }

    println!("Stopped receiving goods.");

}

fn unload_goods(receiver: Receiver<Item>, warehouse: &mut Vec<Option<Item>>) {

    while let Ok(item) = receiver.recv() {
        println!("Received item: {:?}", item);
        warehouse.push(Some(item.clone())); // Add the item to the end of the warehouse vector
    }

}

pub fn inventory_management(){
    // Create a channel to send incoming goods
    let (sender, receiver) = channel::<Item>();

    // Define warehouse storage (vector of Option<Item>)
    let mut warehouse = vec![None; 3]; // Simulates 3 storage locations

    // simualtion time
    let simulation_time = Duration::from_secs(3);
    // Spawn threads for concurrent tasks
    let receive_thread = thread::spawn(move || receive_goods(sender, simulation_time));
    let unload_thread = thread::spawn(move || {
        unload_goods(receiver, &mut warehouse)
    });

    // Wait for all threads to finish
    receive_thread.join().unwrap();
    unload_thread.join().unwrap();

    // listen to orders information
    receiver::activate_receiver();

    println!("Inventory management simulation complete.");
}