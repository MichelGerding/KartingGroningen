// use rocket_dyn_templates::handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, HelperResult, Output};
// use crate::modules::helpers::lap::LapHelper;
// use crate::TemplateData;
//
//
// #[derive(Clone, Copy)]
// pub struct GetSessionAverageHelper;
//
// impl HelperDef for GetSessionAverageHelper {
//     fn call<'reg: 'rc, 'rc>(
//         &self,
//         h: &Helper, _: &Handlebars, _: &Context,
//         _: &mut RenderContext,
//         out: &mut dyn Output) -> HelperResult {
//
//         let time_param = h.param(0);
//         let drivers_param = h.param(1);
//
//
//         if time_param.is_none()  || drivers_param.is_none() {
//             return Ok(());
//         }
//
//         let seconds_in = time_param.unwrap().value().as_f64().unwrap();
//         let drivers_in = drivers_param.unwrap().value().as_array().unwrap();
//
//         let mut data: Vec<TemplateDataDriver> = Vec::new();
//
//         for driver in drivers_in {
//             // parse data and push into array
//             drivers.push(serde_json::from_value(driver.clone()).unwrap());
//         }
//
//         let laps = LapHelper::get_laps_at_time(drivers, seconds_in);
//         out.write(&format!("{:?}", laps))?;
//
//
//         Ok(())
//     }
// }