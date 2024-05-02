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

fn receive_orders_to_check(){
    // let received_orders = format!("{:?}", receiver::activate_receiver());

    let received_orders = receiver::activate_receiver();

    // for order in received_orders {
    //     println!("Received order: {}", order)
    // }

    {// let parts: Vec<&str> = received_orders.split(",").collect();

    // let product_id_str = parts[0].split_whitespace().nth(3).unwrap();
    // let product_id: i32 = product_id_str.parse().unwrap();

    // let customer_id_str = parts[1].split_whitespace().nth(2).unwrap();
    // let customer_id: i32 = customer_id_str.parse().unwrap();

    // let fulfilled_str = parts[2].split_whitespace().nth(1).unwrap();
    // let fulfilled: bool = fulfilled_str.parse().unwrap();

    //  // Print extracted data
    //  println!("Product IDs: {}", product_id);
}

}

pub fn inventory_management(){
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

    receive_thread.join().unwrap();
    unload_thread.join().unwrap();

    receive_orders_to_check();


}