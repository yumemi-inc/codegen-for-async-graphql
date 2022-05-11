use async_graphql_parser::types::TypeDefinition;

use super::{BaseType, FileRender, RenderType};

pub type ScalarTypeWrapper<'a, 'b> = BaseType<'a, 'b, TypeDefinition>;

impl<'a, 'b> FileRender for ScalarTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "scalar_type".to_string()
    }
}

impl<'a, 'b> RenderType for ScalarTypeWrapper<'a, 'b> {
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
