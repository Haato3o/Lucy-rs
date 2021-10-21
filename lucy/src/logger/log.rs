trait Logger {
    fn debug(message: String);
    fn info(message: String);
    fn warn(message: String);
    fn error(message: String);
    fn critical(message: String);
}

pub struct Log {}

impl Logger for Log {
    fn debug(message: String) {
        println!("[DEBG]\t{}", message);
    }

    fn info(message: String) {
        println!("[INFO]\t{}", message);
    }

    fn warn(message: String) {
        println!("[WARN]\t{}", message);
    }

    fn error(message: String) {
        println!("[ERRO]\t{}", message);
    }

    fn critical(message: String) {
        println!("[CRIT]\t{}", message);
    }
}