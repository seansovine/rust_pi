use std::{error::Error, time::Duration};
use ureq::Agent;

fn main() -> Result<(), Box<dyn Error>> {
    // From ureq rustdoc example:
    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build();
    let agent: Agent = config.into();

    let body: String = agent
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .call()?
        .body_mut()
        .read_to_string()?;

    dbg!(body);

    // TODO: Call weather API and parse results.

    Ok(())
}
