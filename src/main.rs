use serde_json::{Value, Number};
use sqlite;
use warp::Filter;

use std::fs::File;
use std::io::BufReader;
use std::time::SystemTime;

fn get_status() -> Value {
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

    v
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let v = get_status();
    let serialized = serde_json::to_string_pretty(&v).unwrap();
    println!("{}", serialized);

    println!("Hello, world!");
    let endpoint = warp::get()
        .map(|| warp::reply::json(&get_status()));
    warp::serve(endpoint).run(([0, 0, 0, 0], 5000)).await;
}
