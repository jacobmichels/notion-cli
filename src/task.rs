use std::fmt::Display;

use colour::{blue, green};

/// A Notion task
#[derive(Debug, Clone)]
pub struct Task {
    /// The task's ID
    pub id: String,
    /// The task's current status
    pub status: TaskStatus,
    /// Title of the task
    pub title: String,
}

impl Task {
    /// Construct a new Task instance with an ID and status
    pub fn new(id: String, status: TaskStatus, title: String) -> Task {
        return Task { id, status, title };
    }

    /// Pretty print a task
    pub fn print(&self, i: usize, with_status: bool, with_id: bool) {
        let mut out = String::new();
        out.push_str(format!("Task: {}\n", self.title).as_str());

        if with_status {
            out.push_str(format!("Status: {}\n", self.status).as_str());
        }

        if with_id {
            out.push_str(format!("ID: {}\n", self.id).as_str());
        }

        if i % 2 == 0 {
            green!("{}", out)
        } else {
            blue!("{}", out);
        }
    }
}

/// The current status of a Notion task
#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum TaskStatus {
    /// Todo: not started
    Todo,
    /// Doing: started but not finished
    Doing,
    /// Done: finished
    Done,
}

impl TaskStatus {
    /// Turn a status into a string representing itself
    /// TaskStatus::Todo -> To Do
    /// TaskStatus::Doing -> Doing
    /// TaskStatus::Done -> Done
    pub fn as_notion_status(&self) -> String {
        return match self {
            TaskStatus::Todo => "To Do".to_string(),
            TaskStatus::Doing => "Doing".to_string(),
            TaskStatus::Done => "Done".to_string(),
        };
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
