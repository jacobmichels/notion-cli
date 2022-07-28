/// Represents a notion database object
pub struct Database {
    /// The title of the database
    title: String,
    /// The ID of the database
    id: String,
    /// True if the database is compatible with the app
    /// To be compatible, a database must have these three statuses: To Do, Doing, and Done
    /// Extra statuses are ignored
    eligible: bool,
}

impl Database {
    pub fn new(id: String, title: String, eligible: bool) -> Database {
        return Database {
            title,
            id,
            eligible,
        };
    }
}
