
#[derive(Debug, thiserror::Error)]
pub enum MermaidParseError {
    #[error("Parse Error: {0}")]
    NomError(String),
}
