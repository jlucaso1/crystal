use crate::Lockfile;
use native_tls::TlsConnector;
use std::sync::mpsc::Receiver;
use std::thread;
use websocket::header::{Authorization, Basic, Headers};
use websocket::{ClientBuilder, Message, OwnedMessage};

pub fn listen(lockfile: &'static Lockfile, rx: Receiver<EventName>) {
  thread::spawn(move || {
    let mut sub = Subscriber {
      status: SubscriberStatus::Idle,
      lockfile,
    };

    sub.connect().unwrap();

    loop {
      match rx.recv() {
        Ok(EventName::Start) => {
          debug!("Starting events listener");
          sub.connect().unwrap();
        }
        Ok(EventName::Restart) => {
          debug!("Restarting events listener");
          sub.connect().unwrap();
        }
        Err(e) => {
          panic!("Event listener broke!: {:?}", e);
        }
      }
    }
  });
}

struct Subscriber {
  status: SubscriberStatus,
  lockfile: &'static Lockfile,
}

impl Subscriber {
  pub fn connect(&mut self) -> Result<(), ()> {
    if self.status == SubscriberStatus::Connected {
      debug!("Attempting to connect, but connection already started");

      return Ok(());
    }

    debug!("Subscribing to League Client");
    let lockfile = self.lockfile.get_details().unwrap();

    if lockfile.is_none() {
      debug!("Lockfile is not ready");
      return Ok(());
    }

    let l = lockfile.unwrap();
    let mut builder = TlsConnector::builder();
    let builder = builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
      username: l.username.clone(),
      password: Some(l.password.clone()),
    }));

    let addr = format!("wss://{}:{}/", l.address, l.port);
    debug!("Connecting to {:?}", addr);

    let mut client = ClientBuilder::new(&addr)
      .unwrap()
      .add_protocol("wamp")
      .custom_headers(&headers)
      .connect_secure(Some(connector))
      .unwrap();
    self.status = SubscriberStatus::Connected;

    let message = Message::text("[5,\"OnJsonApiEvent\"]");
    client.send_message(&message).unwrap();

    for message in client.incoming_messages() {
      let message = match message {
        Ok(m) => m,
        Err(e) => {
          debug!("Error receiving message: {:?}", e);
          self.status = SubscriberStatus::Errored;
          return Ok(());
        }
      };

      match message {
        OwnedMessage::Text(txt) => {
          debug!("Message: {:?}", txt);
        }
        OwnedMessage::Ping(data) => {
          debug!("Ping: {:?}", data);
        }
        OwnedMessage::Pong(data) => {
          debug!("Pong: {:?}", data);
        }
        OwnedMessage::Binary(bin) => {
          debug!("Binary: {:?}", bin);
        }
        OwnedMessage::Close(_) => {
          debug!("Closed connection");
          self.status = SubscriberStatus::Idle;
          return Ok(());
        }
      }
    }

    debug!("Listener done");

    Ok(())
  }
}

#[derive(PartialEq)]
enum SubscriberStatus {
  Idle,
  Connected,
  Errored,
}

pub enum EventName {
  Start,
  Restart,
}
