use chrono::NaiveDateTime;
use inflections::Inflect;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
};

use crate::modules::helpers::handelbars::format_is_child_kart::check_param_count;

/// # general formatting helper
/// a helper to format multiple different types of data to a human readable format
/// to display in the frontend
///
/// ### formats
/// - date's
///
/// ### usage
/// ```handlebars
/// {{format "2021-01-01T00:00:00Z"}}
/// ```
#[derive(Clone, Copy)]
pub struct Format;

impl HelperDef for Format {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        check_param_count(helper, 1)?;
        let a_param = helper.param(0);

        if a_param.is_none() {
            return Ok(());
        }

        let a = a_param.unwrap().value().as_str().unwrap();
        // check if a can be converted to a datetime
        let dt = NaiveDateTime::parse_from_str(a, "%Y-%m-%d %H:%M:%S.%f");

        if dt.is_ok() {
            out.write(&format!("{}", dt.unwrap().format("%A %e %B %Y, %H:%M")))?;
        } else if a.starts_with('-') {
            out.write(a)?;
        } else {
            out.write(&*a.to_title_case())?;
        }

        Ok(())
    }
}
