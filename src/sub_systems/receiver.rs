use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};
use std::thread;


pub fn receiver() -> amiquip::Result<()>{

    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("queue", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    let mut counter = 0;

    loop {

      if counter == 10 {
        break;
      }

      match consumer.receiver().recv() {
        Ok(ConsumerMessage::Delivery(delivery)) => {
          let body = String::from_utf8_lossy(&delivery.body);

          println!("Received: {}", body);
          counter += 1;
          consumer.ack(delivery).unwrap();
        }
        Ok(other) => {
          println!("Consumer ended: {:?}", other);
          break;
        }
        Err(err) => {
          eprintln!("Error receiving message: {}", err);
          break;
        }
      }
    }
    
    connection.close()
}
pub fn activate_receiver() {

    let receiver_handle = thread::spawn(|| {
        if let Err(err) = receiver() {
            eprintln!("Receiver thread error: {}", err);
        }
    });

    receiver_handle.join().unwrap();
}