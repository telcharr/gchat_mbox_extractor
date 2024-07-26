use crate::models::Attachment;

pub fn extract_html_and_attachments(body: &str) -> (String, Vec<Attachment>) {
    let mut html_body = String::new();
    let mut attachments = Vec::new();
    let parts: Vec<&str> = body.split("--").collect();

    for part in parts {
        if part.contains("Content-Type: text/html") {
            html_body = part.to_string();
        } else if part.contains("Content-Disposition: attachment") {
            let attachment = extract_attachment(part);
            if let Some(att) = attachment {
                attachments.push(att);
            }
        }
    }

    (html_body, attachments)
}

fn extract_attachment(part: &str) -> Option<Attachment> {
    let mut content_type = String::new();
    let mut filename = String::new();
    let mut content = Vec::new();

    for line in part.lines() {
        if line.starts_with("Content-Type: ") {
            content_type = line["Content-Type: ".len()..].to_string();
        } else if line.starts_with("Content-Disposition: attachment; filename=") {
            filename = line["Content-Disposition: attachment; filename=".len()..].trim_matches('"').to_string();
        } else if !line.contains(":") {
            content.extend_from_slice(line.as_bytes());
        }
    }

    if !content_type.is_empty() && !filename.is_empty() && !content.is_empty() {
        Some(Attachment {
            content_type,
            filename,
            content,
        })
    } else {
        None
    }
}