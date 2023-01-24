use crate::modules::helpers::handelbars::format_is_child_kart::check_param_count;
use crate::ChartData;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
};

/// # is_child_kart formatting helper
/// a formatter convert chart data to json for use in javascript
///
/// ### usage
/// ```handlebars
/// {{toJson chartData}}
/// ```
#[derive(Clone, Copy)]
pub struct ToJson;

impl HelperDef for ToJson {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        check_param_count(helper, 1)?;
        let json_param = helper.param(0);

        if json_param.is_none() {
            return Ok(());
        }

        let obj: ChartData = serde_json::from_value(json_param.unwrap().value().clone()).unwrap();

        out.write(serde_json::to_string(&obj).unwrap().as_str())?;
        Ok(())
    }
}
