use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
/// # is_child_kart formatting helper
/// a formatter to nicely display if a kart is a child kart or not
///
/// ### usage
/// ```handlebars
/// {{formatChildKart true}}
/// {{formatChildKart false}}
/// ```
#[derive(Clone, Copy)]
pub struct FormatIsChildKartHandlebars;

impl HelperDef for FormatIsChildKartHandlebars {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        check_param_count(helper, 1)?;
        let is_child_kart_param = helper.param(0);

        let is_child_kart: bool =
            serde_json::from_value(is_child_kart_param.unwrap().value().clone()).unwrap();
        if is_child_kart {
            out.write("Child kart")?;
        } else {
            out.write("Adult Kart")?;
        }

        Ok(())
    }
}

pub fn check_param_count(h: &Helper, n: u64) -> Result<(), RenderError> {
    if h.params().len() != n as usize {
        return Err(RenderError::new::<String>(format!(
            "Wrong number of arguments for helper \"{}\", {n} was expected but {} were given",
            h.name(),
            h.params().len()
        )));
    }

    Ok(())
}
