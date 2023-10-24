use std::{env, time::Duration, thread};
use anyhow::Result;
use discord_presence::{Client, Event};

fn main() -> Result<()>{
    dotenv::dotenv().ok();

    let waka_api_key = env::var("WAKA_API_KEY")?;

    let mut client = Client::new(1113765598602727455);
    client.on_event(Event::Ready, |_| {
        println!("READY!");
    });
    let _ = client.start();
    client.block_until_event(Event::Ready)?;
    client.set_activity(|build| build.details("Total time: X // Project time: X").state("Working on discord_code_time"))?;
    thread::sleep(Duration::from_secs(30));

    return Ok(());
}
