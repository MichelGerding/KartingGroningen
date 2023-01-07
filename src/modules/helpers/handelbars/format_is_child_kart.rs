use rocket_dyn_templates::handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, HelperResult, Output};


#[derive(Clone, Copy)]
pub struct FormatChildKart;

impl HelperDef for FormatChildKart {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper, _: &Handlebars, _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output) -> HelperResult {

        let param = h.param(0);

        if param.is_none() {
            return Ok(());
        }

        let is_child_kart: bool = serde_json::from_value(param.unwrap().value().clone()).unwrap();

        if is_child_kart{
            out.write("Child Kart")?;
        } else {
            out.write("Normal Kart")?;
        }

        Ok(())
    }
}