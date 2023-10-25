use std::{env, time::Duration, thread};
use anyhow::Result;
use discord_presence::{Client, Event};

fn main() -> Result<()>{
    dotenv::dotenv().ok();

    let waka_api_key = env::var("WAKA_API_KEY")?;
    let waka_user_id = env::var("WAKA_USER_ID")?;

    let response = reqwest::blocking::get(
        format!("https://wakatime.com/api/v1/users/current/stats?api_key={waka_api_key}")
    )?;
    let json = json::parse(response.text()?.as_str())?;
    let daily_average = &json["data"]["human_readable_daily_average"].as_str().unwrap();

    let mut client = Client::new(1113765598602727455);
    client.on_event(Event::Ready, |_| {
        println!("READY!");
    });
    let _ = client.start();
    client.block_until_event(Event::Ready)?;
    client.set_activity(|build| build.details(format!("Daily average: {daily_average}")).state("Working on discord_code_time"))?;
    thread::sleep(Duration::from_secs(30));

    return Ok(());
}
