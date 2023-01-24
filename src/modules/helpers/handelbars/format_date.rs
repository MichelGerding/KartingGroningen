use crate::modules::helpers::handelbars::format_is_child_kart::check_param_count;
use chrono::NaiveDateTime;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
};

/// # date formatting helper
/// a helper to correctly format a date in the frontend
///
/// ### usage
/// ```handlebars
/// {{formatDate "2021-01-01T00:00:00Z"}}
/// ```
#[derive(Clone, Copy)]
pub struct FormatDateHelper;

impl HelperDef for FormatDateHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        check_param_count(helper, 1)?;
        let date_param = helper.param(0);
        // get the string value

        let date: Result<NaiveDateTime, _> =
            serde_json::from_value(date_param.unwrap().value().clone());

        out.write(&format!("{}", date.unwrap().format("%A %e %B %Y, %H:%M")))?;
        Ok(())
    }
}
