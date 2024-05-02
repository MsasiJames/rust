use std::{thread, time::Duration};

use amiquip::{Connection, Exchange, Publish, QueueDeclareOptions};


pub fn sender(product_name: String, customer_id: u32, fulfilled:bool, process:i32) -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);

    let _ = channel.queue_declare("queue", QueueDeclareOptions::default())?;

    let message_to_check_order = format!("Check Order: {}, customer_id: {}, fulfilled: {}", product_name, customer_id, fulfilled);
    let message_to_confirm_order: String = format!("Confirm Order: {}, {}, {}", product_name, customer_id, fulfilled);

    if process == 1 {
        if let Err(err) = exchange.publish(Publish::new(message_to_check_order.as_bytes(), "queue")) {
            eprintln!("Error sending message: {}", err);
        }else{
            println!("Sent message to inventory for checking: {}", message_to_check_order);
        }
        thread::sleep(Duration::from_millis(2000));
    }else{
        if let Err(err) = exchange.publish(Publish::new(message_to_confirm_order.as_bytes(), "queue")) {
            eprintln!("Error sending message: {}", err);
        }else{
            println!("Sent message to order to confirm: {}", message_to_confirm_order);
        }
        thread::sleep(Duration::from_millis(2000));
    }
    

    connection.close()
}

pub fn activate_sender(product_name: String, customer_id: u32, fulfilled:bool, process:i32) {
    let sender_handle = thread::spawn( move || {
        if let Err(err) = sender(product_name, customer_id, fulfilled, process){
            eprintln!("Sender thread error: {}", err);
        }
    });

    sender_handle.join().unwrap();
}