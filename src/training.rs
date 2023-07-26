use chrono::Local;
use derive_more::Deref;
use derive_more::DerefMut;


#[derive(Debug, Deref, DerefMut)]
pub struct TrainingManager {
    pub output: String
}

impl Default for TrainingManager {
    fn default() -> Self {
        Self { output: "".into() }
    }
}

impl TrainingManager {

    pub fn print(&mut self, msg: &str) {
        let date = Local::now();
        self.output += format!(">{} {}\r\n", date.format("%Y-%m-%d][%H:%M:%S"), msg).as_str();
    }

}