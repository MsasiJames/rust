use std::{thread, time::Duration};

use amiquip::{Connection, Exchange, Publish, QueueDeclareOptions};


pub fn sender(msg: String) -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);

    let _ = channel.queue_declare("queue", QueueDeclareOptions::default())?;

    let message = format!("Message: {:?}", msg);
    if let Err(err) = exchange.publish(Publish::new(message.as_bytes(), "queue")) {
        eprintln!("Error sending message: {}", err);
    }else{
        println!("Sent message: {}", message);
    }
    thread::sleep(Duration::from_millis(2000));

    connection.close()
}

pub fn activate_sender(msg: String) {
    let sender_handle = thread::spawn( || {
        if let Err(err) = sender(msg){
            eprintln!("Sender thread error: {}", err);
        }
    });

    sender_handle.join().unwrap();
}