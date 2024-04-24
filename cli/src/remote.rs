use std::{net::IpAddr, time::Duration};

use anyhow::Result;
use flexispot_e7_controller_web::{RequestPayload, ResponsePayload};
use ureq::{MiddlewareNext, Request};
use url::Url;

use crate::command::Command;

pub fn execute(command: Command, address: IpAddr, port: u16, secret: String) -> Result<()> {
    let client = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(10))
        .timeout_read(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .middleware(move |req: Request, next: MiddlewareNext| {
            next.handle(req.set("Authorization", &secret))
        })
        .build();
    let mut url = Url::parse(&format!("http://{address}:{port}/"))?;

    match command {
        Command::Query => {
            url.set_path("/query");
            let height = client.get(url.as_str()).call()?.into_json::<ResponsePayload>()?;
            println!("Current height: {height} cm");
        }
        _ => {
            client
                .post(url.as_str())
                .send_json(RequestPayload { command: command.into() })?;
        }
    }
    Ok(())
}
