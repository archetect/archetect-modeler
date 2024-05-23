use crate::{mermaid_er_diagram, MermaidErDiagram};
use crate::errors::MermaidParseError;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum MermaidDiagram<'a> {
    EntityRelationship(MermaidErDiagram<'a>),
}

impl<'a> MermaidDiagram<'a> {
    pub fn parse(input: &'a str) -> Result<MermaidDiagram<'a>, MermaidParseError> {
        // TODO: Parse into types based on diagram declaration to support multiple diagrams
        match mermaid_er_diagram(input) {
            Ok((_remaining, diagram))  => {
                Ok(MermaidDiagram::EntityRelationship(diagram))
            }
            Err(err) => {
                Err(MermaidParseError::NomError(err.to_string()))
            }
        }
    }
}
