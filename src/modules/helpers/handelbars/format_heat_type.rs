use crate::modules::helpers::handelbars::format_is_child_kart::check_param_count;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
};

/// # heat_type formatting helper
///
/// ### usage
/// ```handlebars
/// {{formatHeatType "Heat"}}
/// {{formatHeatType "Final"}}
/// ```
///
/// ### output
/// ```
/// Hot Laps
/// Final
/// ```
///
#[derive(Clone, Copy)]
pub struct FormatHeatType;

impl HelperDef for FormatHeatType {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        check_param_count(helper, 1)?;
        let type_param = helper.param(0);

        if type_param.is_none() {
            return Ok(());
        }

        let heat_type: String =
            serde_json::from_value(type_param.unwrap().value().clone()).unwrap();

        if heat_type != "Heat" {
            out.write(&heat_type)?;
        } else {
            out.write("Hot Laps")?;
        }

        Ok(())
    }
}
