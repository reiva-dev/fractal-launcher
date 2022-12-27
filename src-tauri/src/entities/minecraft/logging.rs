#[derive(Debug, serde::Deserialize)]
pub struct Logging {
    client: CliengLogging
}

#[derive(Debug, serde::Deserialize)]
pub struct CliengLogging {
    argument: String,
    file: LoggingFile,
    #[serde(rename = "type")]
    logging_type: String
}

#[derive(Debug, serde::Deserialize)]
pub struct LoggingFile {
    id: String,
    sha1: String,
    size: i64,
    url: String
}