#[derive(Eq, PartialEq, Debug)]
pub struct Event{
    pub topic: String,
    pub start_time: u128
}

pub trait TimeTrackerStore {
    fn prepare(profile_name: &str) -> Self;
    fn add_topic(&mut self, topic: &str, time: u128);
    fn retrieve_events(&mut self) -> Vec<Event>;
}