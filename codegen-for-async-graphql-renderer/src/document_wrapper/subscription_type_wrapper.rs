use async_graphql_parser::types::{FieldDefinition, InputValueDefinition, Type};

use super::{Context, RenderType, SupportField, SupportType, UseContext};

pub struct SubscriptionTypeWrapper<'a, 'b> {
    pub doc: &'a FieldDefinition,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for SubscriptionTypeWrapper<'a, 'b> {
    fn ty(&self) -> &Type {
        &self.doc.ty.node
    }
}

impl<'a, 'b> UseContext for SubscriptionTypeWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> SupportField for SubscriptionTypeWrapper<'a, 'b> {
    fn input_value_types(&self) -> Vec<&InputValueDefinition> {
        let mut res = vec![];
        self.doc.arguments.iter().for_each(|f| res.push(&f.node));
        res
    }
}

impl<'a, 'b> RenderType for SubscriptionTypeWrapper<'a, 'b> {
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
