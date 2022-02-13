

pub fn report_error(message: &str, file_name: &str, line: usize) {
    eprintln!("ERROR {} in {}:{}", message, file_name, line);
}

pub fn report_warning(message: &str, file_name: &str, line: usize) {
    println!("WARNING {} in {}:{}", message, file_name, line);
}