/// This is a test to show if we can easily make patch requests for transip

use serde_json::json;

const STATUS: &str = "status";
const RUNNING: &str = "running";
const STOPPED: &str = "stopped";

fn main() {
    let running = json!({
        STATUS: RUNNING
    });

    let stopped = json!({
        STATUS: STOPPED
    });


    println!("{}", serde_json::to_string(&running).unwrap());
    println!("{}", serde_json::to_string(&stopped).unwrap());
}
