use std::fmt::Display;

/// A Notion task
pub struct Task {
    /// The task's ID
    pub id: String,
    /// The task's current status
    pub status: TaskStatus,
}

/// The current status of a Notion task
#[derive(clap::ValueEnum, Clone)]
pub enum TaskStatus {
    /// Todo: not started
    Todo,
    /// Doing: started but not finished
    Doing,
    /// Done: finished
    Done,
}

impl Task {
    /// Construct a new Task instance with an ID and status
    pub fn new(id: String, status: TaskStatus) -> Task {
        return Task { id, status };
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TaskStatus::Todo => write!(f, "todo")?,
            TaskStatus::Doing => write!(f, "doing")?,
            TaskStatus::Done => write!(f, "done")?,
        };
        return Ok(());
    }
}
