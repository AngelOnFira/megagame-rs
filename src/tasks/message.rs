use super::Task;

pub struct Message {}

impl Task for Message {
    fn run() {
        println!("Message");
    }
}