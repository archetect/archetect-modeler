use archetect_model_mermaid::{DiagramSection, MermaidDiagram, MermaidErDiagram};
use crate::{ArchetectModel, Model};

impl<'a> From<MermaidDiagram<'a>> for crate::model::ArchetectModel {
    fn from(diagram: MermaidDiagram) -> Self {
        match diagram {
            MermaidDiagram::EntityRelationship(er_diagram) => {}
        }
        todo!()
    }
}

fn convert_er_diagram(er_diagram: MermaidErDiagram) -> ArchetectModel {
    let mut am = ArchetectModel::new();

    /// The Mermaid Diagram can describe sections in any order, so we need to insert entities as we
    /// first encounter them, and extend them as we locate each new section.
    let mut model = Model::new();
    for section in er_diagram.sections {
        match section {
            DiagramSection::EntityRelation(relations) => {

            }
            DiagramSection::EntityAttributes(attributes) => {

            }
            DiagramSection::Entity(entity) => {

            }
        }
    }

    am
}
