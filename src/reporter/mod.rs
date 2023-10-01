use crate::lexer::token::TokenSpan;
use ariadne::{Label, Report, ReportKind, Source};

pub fn report_error(message: &str, file_path: &str, span: TokenSpan, opt_label: Option<&str>) {
    let file_content = std::fs::read_to_string(file_path);

    if file_content.is_err() {
        eprintln!("Error: {} {}:{}-{} {}", message, file_path, span.start, span.end, opt_label.unwrap_or(""));
        return;
    }

    let file_content = file_content.unwrap();

    Report::build(ReportKind::Error, file_path, span.start)
        .with_message(message)
        .with_label(
            Label::new((file_path, span.start..span.end)).with_message(opt_label.unwrap_or("")),
        )
        .finish()
        .print((file_path, Source::from(file_content.as_str())))
        .unwrap();
}

#[allow(dead_code)]
pub fn report_warning(message: &str, file_path: &str, span: TokenSpan, opt_label: Option<&str>) {
    let file_content = std::fs::read_to_string(file_path);

    if file_content.is_err() {
        println!("Warning: {} {}:{}-{} {}", message, file_path, span.start, span.end, opt_label.unwrap_or(""));
        return;
    }

    let file_content = file_content.unwrap();

    Report::build(ReportKind::Error, file_path, span.start)
        .with_message(message)
        .with_label(
            Label::new((file_path, span.start..span.end)).with_message(opt_label.unwrap_or("")),
        )
        .finish()
        .print((file_path, Source::from(file_content.as_str())))
        .unwrap();
}
