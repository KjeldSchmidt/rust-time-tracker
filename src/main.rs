mod file_database;
mod time_tracker_store;

use crate::file_database::FileDatabase;
use crate::time_tracker_store::TimeTrackerStore;
use std::io::{self};
use std::time::{UNIX_EPOCH, SystemTime};

fn main() {
    let mut file_database = FileDatabase::prepare("default");
    loop {
        let s = get_topic_name_input();
        let now_utc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        file_database.add_topic( &*s, now_utc);
    }
}

fn get_topic_name_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("Which topic is tracked now?");
    stdin.read_line(&mut buffer).expect("No std read possible");
    return buffer;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}