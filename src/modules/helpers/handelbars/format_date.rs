use chrono::NaiveDateTime;
use rocket_dyn_templates::handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, HelperResult, Output};


#[derive(Clone, Copy)]
pub struct FormatDateHelper;

impl HelperDef for FormatDateHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper, _: &Handlebars, _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output) -> HelperResult {

        let date_param = h.param(0);

        if date_param.is_none() {
            return Ok(());
        }

        let date: NaiveDateTime = serde_json::from_value(date_param.unwrap().value().clone()).unwrap();
        out.write(&format!("{}", date.format("%A %e %B %Y, %H:%M")))?;

        Ok(())
    }
}