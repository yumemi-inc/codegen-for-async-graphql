use super::Config;
use async_graphql_parser::types::{
    ServiceDocument, TypeDefinition, TypeKind, TypeSystemDefinition,
};
use std::collections::HashMap;

use crate::document_wrapper::{
    FileRender, InputObjectTypeWrapper, InterfaceTypeWrapper, MutationsTypeWrapper, ObjectPath,
    ObjectTypeWrapper, RenderType, ScalarTypeWrapper, SubscriptionRootTypeWrapper,
    UnionTypeWrapper,
};

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub config: &'a Config,
    doc: &'a ServiceDocument,
}

fn get_paths<T>(obj: &[T]) -> Vec<ObjectPath>
where
    T: FileRender,
{
    obj.iter().map(|f| f.path()).collect()
}

impl<'a> Context<'a> {
    #[must_use]
    pub const fn new(config: &'a Config, doc: &'a ServiceDocument) -> Self {
        Self { config, doc }
    }

    #[must_use]
    pub fn scalar_names(&self) -> Vec<String> {
        self.scalar_types()
            .iter()
            .map(ScalarTypeWrapper::name)
            .collect()
    }

    #[must_use]
    pub fn union_names(&self) -> Vec<String> {
        self.union_types()
            .iter()
            .map(UnionTypeWrapper::name)
            .collect()
    }

    pub fn input_object_type_names(&self) -> Vec<String> {
        self.input_object_types()
            .iter()
            .map(InputObjectTypeWrapper::name)
            .collect()
    }

    pub fn file_paths(&self) -> Vec<ObjectPath> {
        vec![
            self.scalar_file_paths(),
            self.object_type_file_paths(),
            self.interface_type_file_paths(),
            self.mutation_type_file_paths(),
            self.input_object_type_file_paths(),
            self.union_type_file_paths(),
            self.subscription_type_file_paths(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn structured_file_paths(&self) -> HashMap<String, Vec<ObjectPath>> {
        let mut map = HashMap::new();

        self.file_paths().iter().for_each(|f| {
            let r = vec![];
            map.entry(f.super_module_name.clone())
                .or_insert_with(|| r)
                .push(f.clone());
        });
        map
    }

    fn scalar_file_paths(&self) -> Vec<ObjectPath> {
        get_paths(&self.scalar_types())
    }

    fn object_type_file_paths(&self) -> Vec<ObjectPath> {
        get_paths(&self.object_types())
    }

    fn interface_type_file_paths(&self) -> Vec<ObjectPath> {
        get_paths(&self.interface_types())
    }

    fn mutation_type_file_paths(&self) -> Vec<ObjectPath> {
        get_paths(&self.mutation_types())
    }

    fn subscription_type_file_paths(&self) -> Vec<ObjectPath> {
        get_paths(&self.subscription_types())
    }

    fn input_object_type_file_paths(&self) -> Vec<ObjectPath> {
        get_paths(&self.input_object_types())
    }

    fn union_type_file_paths(&self) -> Vec<ObjectPath> {
        get_paths(&self.union_types())
    }

    fn type_definition(&self) -> Vec<&TypeDefinition> {
        self.doc
            .definitions
            .iter()
            .filter_map(|f| match &f {
                TypeSystemDefinition::Type(n) => Some(&n.node),
                TypeSystemDefinition::Schema(_n) => None,
                _ => panic!("Not implemented:{:?}", f),
            })
            .collect()
    }

    #[must_use]
    pub fn mutation_types(&self) -> Vec<MutationsTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f.kind {
                TypeKind::Object(object) => {
                    if f.name.node == "Mutation" {
                        return Some(MutationsTypeWrapper {
                            kind: object,
                            doc: f,
                            context: self,
                        });
                    }
                    None
                }
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn subscription_types(&self) -> Vec<SubscriptionRootTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f.kind {
                TypeKind::Object(object) => {
                    if f.name.node == "Subscription" {
                        return Some(SubscriptionRootTypeWrapper {
                            kind: object,
                            doc: f,
                            context: self,
                        });
                    }
                    None
                }
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn object_types(&self) -> Vec<ObjectTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f.kind {
                TypeKind::Object(object) => {
                    if f.name.node == "Mutation" {
                        return None;
                    }
                    if f.name.node == "Subscription" {
                        return None;
                    }
                    Some(ObjectTypeWrapper {
                        kind: object,
                        doc: f,
                        context: self,
                    })
                }
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn scalar_types(&self) -> Vec<ScalarTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f.kind {
                TypeKind::Scalar => Some(ScalarTypeWrapper {
                    kind: f,
                    doc: f,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn union_types(&self) -> Vec<UnionTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f.kind {
                TypeKind::Union(union) => Some(UnionTypeWrapper {
                    kind: union,
                    doc: &f,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn interface_types(&self) -> Vec<InterfaceTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f.kind {
                TypeKind::Interface(interface) => Some(InterfaceTypeWrapper {
                    kind: interface,
                    doc: &f,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn input_object_types(&self) -> Vec<InputObjectTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f.kind {
                TypeKind::InputObject(input_object) => Some(InputObjectTypeWrapper {
                    kind: input_object,
                    doc: &f,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }
}
