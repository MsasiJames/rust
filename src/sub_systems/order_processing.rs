use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use rand::{thread_rng, Rng};

use crate::sub_systems::sender;


#[derive(Debug)]
struct Order {
    customer_id: u32,
    product_name: String,
    fulfilled: bool,
}

fn receive_orders(tx: Sender<Order>) -> thread::JoinHandle<()> {
    // Receiving orders simulation
    thread::spawn(move || {
        let mut rng = thread_rng();
        for i in 0..10{
            let customer_id = i + 1;
            let product_id = rng.gen_range(1000..=1005);
            let product_name = format!("Product {}", product_id);
            println!("Customer {} sent order for {}", customer_id, product_name);
            let order = Order {
                customer_id,
                product_name,
                fulfilled: false,
            };

            match tx.send(order) {
                Ok(_) => {},
                Err(e) => println!("Error: {}", e),
            }
            thread::sleep(std::time::Duration::from_secs(1));
        }
    })
}

fn process_orders(rx_orders: Receiver<Order>, tx_packing: Sender<Order>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let order = match rx_orders.recv() {
            Ok(order) => order,
            Err(err) => {
                println!("Error receiving order: {:?}", err);
                // Handle receive error gracefully, e.g., break the loop or return early
                break;
            }
        };
        match order.fulfilled {
            false => {
                println!("Received order: {:?}", order);
                sender::activate_sender(order.product_name.clone());    // sends order to inventory
                thread::sleep(std::time::Duration::from_secs(1));
            }
            true => {
                println!("No more orders to process");
                break;
            }
        }
        if let Err(err) = tx_packing.send(order) {
            println!("Error sending order to packing: {:?}", err);
            break;
        }
    })
}

fn pack_orders(rx_packing: Receiver<Order>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop{
        let order = match rx_packing.recv() {
            Ok(order) => order,
            Err(err) => {
                println!("Error receiving order: {:?}", err);
                break;
            }
        };
        if order.fulfilled == false {
            println!("Order for customer {} (product: {}) packed and ready for shipment!", order.customer_id, order.product_name);
        } else {
            break;
        }
    })
}

pub fn order_management(){
    let (tx_orders, rx_orders) = channel();
    let (tx_packing, rx_packing) = channel();

    let receive_handle = receive_orders(tx_orders);
    let process_orders = process_orders(rx_orders, tx_packing);
    let packing_orders = pack_orders(rx_packing);

    receive_handle.join().unwrap();
    process_orders.join().unwrap();
    packing_orders.join().unwrap();

    
}

