use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchetectModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    applications: Option<LinkedHashMap<String, Application>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<Model>,
}

impl ArchetectModel {
    pub fn new() -> ArchetectModel {
        ArchetectModel { applications: None, model: None }
    }

    pub fn set_application<K: Into<String>>(&mut self, key: K, application: Application) -> &Self {
        self.applications.get_or_insert_with(|| LinkedHashMap::new())
            .insert(key.into(), application);
        self
    }

    pub fn with_application<K: Into<String>>(mut self, key: K, application: Application) -> Self {
        self.set_application(key, application);
        self
    }

    pub fn set_model(&mut self, model: Model) -> &Self {
        self.model = Some(model);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.set_model(model);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    #[serde(alias = "project-prefix")]
    prefix: String,
    #[serde(alias = "project-suffix")]
    suffix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<Model>,
}

impl Application {
    pub fn new<P: Into<String>, S: Into<String>>(prefix: P, suffix: S) -> Self {
        Self {
            prefix: prefix.into(),
            suffix: suffix.into(),
            language: None,
            transport: None,
            model: None,
        }
    }

    pub fn set_language<L: Into<String>>(&mut self, language: L) -> &mut Self {
        self.language = Some(language.into());
        self
    }

    pub fn with_language<L: Into<String>>(mut self, language: L) -> Self {
        self.set_language(language);
        self
    }

    pub fn set_transport<L: Into<String>>(&mut self, transport: L) -> &mut Self {
        self.transport = Some(transport.into());
        self
    }

    pub fn with_transport<L: Into<String>>(mut self, transport: L) -> Self {
        self.set_transport(transport);
        self
    }

    pub fn set_model(&mut self, model: Model) -> &mut Self {
        self.model = Some(model);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.set_model(model);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    entities: LinkedHashMap<String, Entity>,
}

impl Model {
    pub fn new() -> Self {
        Self { entities: Default::default() }
    }

    pub fn set_entity<K: Into<String>>(&mut self, key: K, entity: Entity) -> &mut Self {
        self.entities.insert(key.into(), entity);
        self
    }

    pub fn with_entity<K: Into<String>>(mut self, key: K, entity: Entity) -> Self {
        self.set_entity(key, entity);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub fields: LinkedHashMap<String, Field>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    name: String,
    data_type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    String,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Bytes,
    Ref {
        /// When set, this refers to an Entity in an adjacent Application, otherwise it refers
        /// to an adjacent Entity in the same application
        #[serde(skip_serializing_if = "Option::is_none")]
        application: Option<String>,
        entity: String,
        field: String,
    },
}
