use async_graphql_parser::types::ObjectType;

use super::{BaseType, FieldWrapper, FileRender, RenderType, SupportFields};

pub type ObjectTypeWrapper<'a, 'b> = BaseType<'a, 'b, ObjectType>;

impl<'a, 'b> FileRender for ObjectTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "object_type".to_string()
    }
}

impl<'a, 'b> RenderType for ObjectTypeWrapper<'a, 'b> {
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

impl<'a, 'b> SupportFields for ObjectTypeWrapper<'a, 'b> {
    #[must_use]
    fn fields(&self) -> Vec<FieldWrapper> {
        self.kind
            .fields
            .iter()
            .map(|f| FieldWrapper {
                doc: &f.node,
                context: self.context,
            })
            .collect()
    }
}

impl<'a, 'b> ObjectTypeWrapper<'a, 'b> {
    pub fn implements_interfaces(&self) -> Vec<String> {
        self.kind
            .implements
            .iter()
            .map(|f| f.node.to_string())
            .collect()
    }
}
