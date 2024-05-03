//  Example 1

/*
use zbus::{Connection, Result, proxy};
use async_std::stream::StreamExt;

#[proxy(
    interface = "org.zbus.MyGreeter1",
    default_service = "org.zbus.MyGreeter",
    default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
    #[zbus(signal)]
    async fn get_random_name(&self) -> Result<String>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `proxy` macro creates `MyGreeterProxy` based on `Notifications` trait.
    let proxy = MyGreeterProxy::new(&connection).await?;

    // // Call say_hello as before
    // let reply = proxy.say_hello("Maria").await?;
    // println!("{reply}");

    // Call get_random_name to retrieve the latest random name
    // let random_name = proxy.get_random_name().await?;

    let mut new_random_name_stream = proxy.receive_get_random_name().await?;
    // println!("Latest random name: {}", random_name);
    while let Some(new_random_name) = new_random_name_stream.next().await {
        println!("Latest random name: {:?}", new_random_name);
    }

    Ok(())
}

*/

// Example 2
use async_std::stream::StreamExt;
use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.zbus.MyGreeter1",
    default_service = "org.zbus.MyGreeter",
    default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
    #[zbus(signal)]
    async fn random_number(&self) -> Result<u32>;
}

#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    let proxy = MyGreeterProxy::new(&connection).await?;

    let mut random_number_stream = proxy.receive_random_number().await?;
    while let Some(random_number) = random_number_stream.next().await {
        println!("Latest random number: {:?}", random_number);
    }

    Ok(())
}
