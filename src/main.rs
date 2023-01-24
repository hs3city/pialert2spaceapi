use serde_json::{Value, Number, json};
use sqlite;
use warp::Filter;

use std::fs::File;
use std::io::{BufReader, Read};
use std::net::TcpStream;
use std::time::SystemTime;

fn get_status(counter_info: &(String, u16)) -> Value {
    let connection = sqlite::open("pialert.db").unwrap();
    let mut cursor = connection.prepare("SELECT COUNT(*) FROM Devices WHERE dev_Archived=0 AND dev_PresentLastScan=1 AND dev_Owner!='House'").unwrap().into_cursor();
    let mut count = 0;

    while let Some(Ok(row)) = cursor.next() {
        count = row.get::<i64, _>(0);
    }

    let file = File::open("spaceapi.json").unwrap();
    let reader = BufReader::new(file);
    let mut v: Value = serde_json::from_reader(reader).unwrap();
    if count > 0 {
        v["state"]["open"] = Value::Bool(true);
    }
    v["state"]["lastchange"] = Value::Number(Number::from(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()));

    let mut stream = TcpStream::connect((counter_info.0.as_str(), counter_info.1)).unwrap();

    let mut data = String::new();
    stream.read_to_string(&mut data).unwrap();
    let counts: Value = serde_json::from_str(&data).unwrap();

    let persons = counts["persons"].as_i64().unwrap();
    v["sensors"]["people_now_present"] = json!([{"value": Value::Number(Number::from(persons))}]);
    if persons > 0 {
        v["state"]["open"] = Value::Bool(true);
    }

    v
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let counter_host = std::env::var("COUNTER_HOST").unwrap_or("counter.local".to_string());
    let counter_port = 26178;
    let counter_info = (counter_host, counter_port);

    let v = get_status(&counter_info);
    let serialized = serde_json::to_string_pretty(&v).unwrap();
    println!("{}", serialized);

    let endpoint = warp::get()
        .map(move || warp::reply::json(&get_status(&counter_info)));
    warp::serve(endpoint).run(([0, 0, 0, 0], 5000)).await;
}
