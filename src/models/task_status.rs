use std::fmt::Display;

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
