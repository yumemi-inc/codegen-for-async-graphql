use async_graphql_parser::types::{InputObjectType, InputValueDefinition, TypeDefinition};

use super::{Context, FileRender, RenderType, SupportField, UseContext};

#[derive(Debug, Clone)]
pub struct InputObjectTypeWrapper<'a, 'b> {
    pub kind: &'a InputObjectType,
    pub doc: &'a TypeDefinition,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for InputObjectTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "input_object_type".to_string()
    }
}

impl<'a, 'b> RenderType for InputObjectTypeWrapper<'a, 'b> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.to_string()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }
}

impl<'a, 'b> UseContext for InputObjectTypeWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> SupportField for InputObjectTypeWrapper<'a, 'b> {
    fn input_value_types(&self) -> Vec<&InputValueDefinition> {
        let mut res = vec![];
        self.kind.fields.iter().for_each(|f| res.push(&f.node));
        res
    }
}
