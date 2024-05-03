//  Example 1

// use anyhow::Result;
// use event_listener::{Event, Listener};
// use zbus::{fdo, interface, SignalContext};

// struct Greeter {
//     name: String,
//     done: Event,
// }

// #[interface(name = "org.zbus.MyGreeter1")]
// impl Greeter {
//     async fn say_hello(&self, name: &str) -> String {
//         format!("Hello {}!", name)
//     }

//     // Rude!
//     async fn go_away(&self, #[zbus(signal_context)] ctxt: SignalContext<'_>) -> fdo::Result<()> {
//         match Self::greeted_everyone(&ctxt).await {
//             Ok(_) => {
//                 self.done.notify(1);
//                 Ok(())
//             }
//             Err(e) => {
//                 // Handle the error here. You can either log it or convert it into a `zbus::fdo::Error`.
//                 // For now, let's just log it and return an `Ok(())`.
//                 eprintln!("Error when greeting everyone: {:?}", e);
//                 Ok(())
//             }
//         }
//     }

//     /// A "GreeterName" property.
//     #[zbus(property)]
//     async fn greeter_name(&self) -> &str {
//         &self.name
//     }

//     /// A setter for the "GreeterName" property.
//     ///
//     /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
//     /// notify listeners that "GreeterName" was updated. It will be automatically called when
//     /// using this setter.
//     #[zbus(property)]
//     async fn set_greeter_name(&mut self, name: String) {
//         self.name = name;
//     }

//     /// A signal; the implementation is provided by the macro.
//     #[zbus(signal)]
//     async fn greeted_everyone(ctxt: &SignalContext<'_>) -> zbus::Result<()>;
// }

// // Although we use `async-std` here, you can use any async runtime of choice.
// #[async_std::main]
// async fn main() -> Result<()> {
//     let greeter = Greeter {
//         name: "GreeterName".to_string(),
//         done: event_listener::Event::new(),
//     };
//     let done_listener = greeter.done.listen();
//     let _connection = zbus::connection::Builder::session()?
//         .name("org.zbus.MyGreeter")?
//         .serve_at("/org/zbus/MyGreeter", greeter)?
//         .build()
//         .await?;

//     done_listener.wait();

//     Ok(())
// }



//  Example 2

use std::time::Duration;
use anyhow::Result;
use async_std::task;
use rand::Rng;
use zbus::{fdo, interface, Connection, SignalContext};

struct Greeter;

#[interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    async fn random_number(&self) -> u32 {
        let random_number: u32 = rand::thread_rng().gen_range(0..100);
        random_number
    }

    #[zbus(property)]
    async fn greeter_name(&self) -> u32 {
        self.random_number().await
    }

    #[zbus(signal)]
    async fn generate_random(ctxt: &SignalContext<'_>) -> zbus::Result<()>;

    async fn generate_random_notify(
        &self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> fdo::Result<()> {
        Self::generate_random(&ctxt).await?;
        Ok(())
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    // setup the server
    connection
        .object_server()
        .at("/org/zbus/MyGreeter", Greeter)
        .await?;
    // before requesting the name
    connection.request_name("org.zbus.MyGreeter").await?;

    // connection.

    loop {
        // do something else, wait forever or timeout here:
        // handling D-Bus messages is done in the background
        task::sleep(Duration::from_secs(1)).await;
        let random_number = Greeter.greeter_name().await;
        println!("Generated random number: {}", random_number);

        // notify the clients
        let greeter_ref = connection
            .object_server()
            .interface::<_, Greeter>("/org/zbus/MyGreeter")
            .await?;
        let mut ctxt = greeter_ref.get_mut().await;

        ctxt.greeter_name_changed(greeter_ref.signal_context())
            .await?;

        std::future::pending::<()>().await;
    }
}



//  Example 3

// use zbus::{connection::Builder, fdo, interface, object_server::SignalContext, Result};

// use event_listener::{Event, Listener};

// struct Greeter {
//     name: String,
//     done: Event,
// }

// #[interface(name = "org.zbus.MyGreeter1")]
// impl Greeter {
//     async fn say_hello(&self, name: &str) -> String {
//         format!("Hello {}!", name)
//     }

//     // Rude!
//     async fn go_away(&self, #[zbus(signal_context)] ctxt: SignalContext<'_>) -> fdo::Result<()> {
//         Self::greeted_everyone(&ctxt).await?;
//         self.done.notify(1);

//         Ok(())
//     }

//     /// A "GreeterName" property.
//     #[zbus(property)]
//     async fn greeter_name(&self) -> &str {
//         &self.name
//     }

//     /// A setter for the "GreeterName" property.
//     ///
//     /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
//     /// notify listeners that "GreeterName" was updated. It will be automatically called when
//     /// using this setter.
//     #[zbus(property)]
//     async fn set_greeter_name(&mut self, name: String) {
//         self.name = name;
//     }

//     /// A signal; the implementation is provided by the macro.
//     #[zbus(signal)]
//     async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
// }

// // Although we use `async-std` here, you can use any async runtime of choice.
// #[async_std::main]
// async fn main() -> Result<()> {
//     let greeter = Greeter {
//         name: "GreeterName".to_string(),
//         done: event_listener::Event::new(),
//     };
//     let done_listener = greeter.done.listen();
//     let _connection = Builder::session()?
//         .name("org.zbus.MyGreeter")?
//         .serve_at("/org/zbus/MyGreeter", greeter)?
//         .build()
//         .await?;

//     done_listener.wait();

//     Ok(())
// }
