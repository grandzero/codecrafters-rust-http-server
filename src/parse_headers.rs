#[derive(Debug)]
pub enum Headers {
    UserAgent(String),
    ContentType(String),
    ContentLength(String),
    Host(String),
    Body(String),
    AcceptEncoding(String),
    Protocol(String),
}

pub fn parse_headers(request_details_as_str: &str) -> Vec<Headers> {
    let request_lines = request_details_as_str.split("\r\n");
    let mut headers: Vec<Headers> = Vec::new();
    request_lines.for_each(|line| {
        if line.contains("User-Agent") {
            headers.push(Headers::UserAgent(
                line.split(":").collect::<Vec<&str>>()[1].trim().to_string(),
            ));
        } else if line.contains("HTTP") {
            headers.push(Headers::Protocol(
                line.split(" ").collect::<Vec<&str>>()[0].to_string(),
            ));
        } else if line.contains("Content-Type") {
            headers.push(Headers::ContentType(
                line.split(":").collect::<Vec<&str>>()[1].trim().to_string(),
            ));
        } else if line.contains("Content-Length") {
            headers.push(Headers::ContentLength(
                line.split(":").collect::<Vec<&str>>()[1].trim().to_string(),
            ));
        } else if line.contains("Host") {
            headers.push(Headers::Host(
                line.split(":").collect::<Vec<&str>>()[1].trim().to_string(),
            ));
        } else if line.contains("Accept-Encoding") {
            headers.push(Headers::AcceptEncoding(
                line.split(":").collect::<Vec<&str>>()[1].trim().to_string(),
            ));
        } else {
            if line.len() >= 1 {
                headers.push(Headers::Body(line.to_string()));
            }
        }
    });
    headers
}
