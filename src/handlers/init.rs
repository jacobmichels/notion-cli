use crate::cli::InitHandler;

pub struct Init {}

impl InitHandler for Init {
    fn init(&self) {
        println!("Init command called")
    }
}
