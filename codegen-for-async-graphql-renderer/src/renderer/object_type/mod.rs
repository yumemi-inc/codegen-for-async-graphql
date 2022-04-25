mod renderer;

use proc_macro2::TokenStream;

use super::{
    Context, FieldRenderer, FileRender, ObjectTypeWrapper, Output, RenderDependencies, RenderType,
    Save, SupportFields,
};

use renderer::Renderer;

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().object_types().iter().for_each(|f| {
            Renderer::create_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .object_types()
            .iter()
            .map(Renderer::new_and_token_stream)
            .collect()
    }
}
