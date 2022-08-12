use colour::{blue, green};

use super::task_status::TaskStatus;

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
