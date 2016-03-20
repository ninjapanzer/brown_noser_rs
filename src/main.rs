extern crate github;

// Example of some public requests

use github::Client;
use github::error::*;
use github::activity::events::*;
use github::pull_requests::*;

fn main() {
  let client = &Client::new("ninjapanzer");
  println!("# Example: list_events");
  match list_events(client) {
    Ok((events, resp)) => {
      println!("listing public events successful, we have {} requests remaining of {}. Limit resets @ {}...",
        resp.rate.remaining, resp.rate.limit, resp.rate.reset);
      for event in events {
        println!("event #{} at {} by {}...",
          event.id, event.created_at, event.actor.login);
      }
    }
    Err(err) => {
      println!("list_events => {}", err);
      if let ClientError::Http(http_error) = err {
        for error in http_error.errors {
          println!("    {}", error);
        }
      }
    }
  }

  match list_pulls(client, "WCCCEDU", "CPT-180-27-Assignment-5-Structures"){
    Ok(thing) => {
      println!("{:?}", thing);
    },
    Err(err) => {
      println!("list_pulls => {:?}", err);
      if let ClientError::Http(http_error) = err {
        for error in http_error.errors {
          println!("    {}", error);
        }
      }
    },
  }
}
