use colour::{blue_ln, green_ln};

/// Represents a notion database object
pub struct Database {
    /// The title of the database
    title: String,
    /// The ID of the database
    id: String,
}

impl Database {
    /// Return a new Database
    pub fn new(id: String, title: String) -> Database {
        return Database { title, id };
    }

    /// Print a database in alternating colours
    pub fn print(&self, i: usize) {
        if i % 2 == 0 {
            green_ln!("Database: {} | ID: {}", self.title, self.id)
        } else {
            blue_ln!("Database: {} | ID: {}", self.title, self.id);
        }
    }
}
