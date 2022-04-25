mod renderer;

use super::{Context, FileRender, Output, RenderDependencies, Save, UnionTypeWrapper};

use renderer::Renderer;

use proc_macro2::TokenStream;

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().union_types().iter().for_each(|f| {
            Renderer::create_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .union_types()
            .iter()
            .map(Renderer::new_and_token_stream)
            .collect()
    }
}
