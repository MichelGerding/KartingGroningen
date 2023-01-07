use rocket_dyn_templates::handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, HelperResult, Output};


#[derive(Clone, Copy)]
pub struct FormatHeatType;

impl HelperDef for FormatHeatType {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper, _: &Handlebars, _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output) -> HelperResult {

        let type_param = h.param(0);

        if type_param.is_none() {
            return Ok(());
        }

        let heat_type: String = serde_json::from_value(type_param.unwrap().value().clone()).unwrap();

        if heat_type != "Heat" {
            out.write(&heat_type)?;
        } else {
            out.write("Hot Laps")?;
        }

        Ok(())
    }
}