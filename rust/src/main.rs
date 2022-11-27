use std::{env, time, thread};
use rand::Rng;
use log::{info, error};
use chrono::{Utc, Duration, NaiveDate, Datelike};
use reqwest::blocking;
use std::error::Error;
use serde_json::{Value, from_str};
use std::collections::{HashSet, HashMap};
use serde::{Serialize, Deserialize};
use thirtyfour::prelude::*;
use thirtyfour::extensions::query::conditions;
use tokio;



#[derive(Serialize, Deserialize, Debug)]
struct Session {
    Capacity: i32,
    Cost: Option<f32>,
    StartTime: i32,
    EndTime: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    Date: String,
    Sessions: Vec<Session>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Resource {
    Name: String,
    Days: Vec<Day>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Resources {
    Resources: Vec<Resource>,
}


#[tokio::main]
async fn send_request(target: String, message_txt: String) -> WebDriverResult<()>{
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--user-data-dir=chrome-data");
    caps.add_chrome_option("excludeSwitches", ["enable-automation"]);
    caps.add_chrome_option("useAutomationExtension", false);
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://web.whatsapp.com").await?;
    let x_arg = format!("//span[contains(@title,'{}')]", target);
    let group_title = driver.query(By::XPath(&x_arg)).first().await?;
    group_title.wait_until().displayed().await?;
    info!("group! {}", group_title);
    info!("wait for a few seconds");
    group_title.click().await?;
    let xpath_message_box = driver.query(
        By::XPath("//*[@id=\"main\"]/footer/div[1]/div/span[2]/div/div[2]/div[1]/div/div[1]")
        ).first().await?;
    xpath_message_box.wait_until().displayed().await?;
    xpath_message_box.send_keys(message_txt).await?;
    thread::sleep(time::Duration::from_secs(10));
    driver.quit().await?;
    Ok(())
}

fn scrape(prev: HashSet<(String, String, i32, i32, i32)>) -> Result<HashSet<(String, String, i32, i32, i32)>, Box<dyn Error>>{
    let date_now = Utc::now().date_naive();
    let date_next_week = date_now + Duration::days(7);
    let url = format!("https://clubspark.lta.org.uk/v0/VenueBooking/StJohnsParkLondon/GetVenueSessions?resourceID=&startDate={}&endDate={}&roleId=", date_now, date_next_week);
    info!("{}", url);
    let resp = blocking::get(url)?.text()?;
    let resp_json: Resources = from_str(&resp)?;
    info!("resp?");
    // info!("resp{}", resp_json.Resources);

    let mut all_availables = HashSet::new();
    for r in resp_json.Resources {
        let court = r.Name;
        info!("court {}", court);
        for d in r.Days {
            let available_date = NaiveDate::parse_from_str(&d.Date, "%Y-%m-%dT%H:%M:%S")?;
            info!("day: {}", available_date);
            for s in d.Sessions {
                let capacity = s.Capacity;
                if capacity <= 0 {
                    continue;
                }
                match s.Cost {
                    None => (),
                    Some(cost) => {
                        let from_hour = s.StartTime / 60;
                        let to_hour = s.EndTime / 60;
                        info!("from_hour {}", from_hour);
                        info!("? {}", available_date.weekday().num_days_from_monday());
                        if available_date.weekday().num_days_from_monday() >= 4 || to_hour > 18{
                            all_availables.insert((court.clone(), available_date.to_string(), from_hour, to_hour, cost as i32));
                        }
                    }
                }
            }
        }
    }
    if !prev.eq(&all_availables) {
        if all_availables.len() > 0 {
            info!("{} slots available, sending updates", all_availables.len());
            match send_request("Tennis bot".to_string(), set_to_str(date_now, all_availables.clone())) {
                Ok(o) => {
                    info!("selenium ok");
                }
                Err(err) => {
                    error!("err! {}", err);
                }
            }
        } else {
            info!("all slots unavailable :(");
            match send_request("Tennis bot".to_string(), "all slots unavailable :(\n".to_string()) {
                Ok(o) => {
                    info!("selenium ok");
                }
                Err(err) => {
                    error!("err! {}", err);
                }
            }
        }
    }
    Ok(all_availables)
}
fn set_to_str(date_now: NaiveDate, all_availables: HashSet<(String, String, i32, i32, i32)>) -> String {
    let mut ret = String::from("hey, new updates!\n");
    for d in all_availables {
        ret.push_str(&format!("{}:00 to {}:00 on {}, {}, cost: {}\n", d.2, d.3, d.1, d.0, d.4))
    }
    ret.push_str(&format!("book here: https://clubspark.lta.org.uk/StJohnsParkLondon/Booking/BookByDate#?date={}&role=guest\n", date_now));
    ret
}
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let target = &args[1];
    info!("{}", target);
    let mut prev = HashSet::new();
    loop {
        prev = match scrape(prev) {
            Ok(new_availables) => {
                info!("ok!");
                new_availables
            }
            Err(r) => {
                error!("{}", r);
                HashSet::new()
            }
        };
        let mut rng = rand::thread_rng();
        let sleep_time = rng.gen_range(60*5..60*15);
        info!("sleep for {}s ({}mins)", sleep_time, sleep_time / 60);
        thread::sleep(time::Duration::from_secs(sleep_time));
    }
}
