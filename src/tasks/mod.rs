// use self::message::Message;

// mod message;

pub enum Tasks {
    Message(Message),
    ChangeTeam,
    CreateRole,
    CreateCategory,
    CreateTeamChannel,
    CreateTeamVoiceChannel,
    CreateCategoryChannel,
    CreateButtons,
    CreateMessage,
    CreateThread,
}

struct TaskRunner {}

struct Task {
    task: Tasks,
}

struct Message {
    player_id: String,
    message: String,
}

impl Task {
    fn message(&self) {
        let message = if let Tasks::Message(message) = &self.task {
            message
        } else {
            panic!("Not a message task");
        };

        
    }
}

impl TaskRunner {
    pub fn run_tasks(&self) {
        // Iterate through open tasks in the DB

        let task = Task {
            task: Tasks::Message(Message {
                player_id: String::from(""),
                message: String::from(""),
            }),
        };

        match task.task {
            Tasks::Message(_) => task.message(),
            _ => unimplemented!(),
        }
    }
}

// pub trait Task {
//     fn run();
// }
