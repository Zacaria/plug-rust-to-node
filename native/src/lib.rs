#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::vm::{Call, JsResult};
use neon::js::JsNumber;

fn cpus(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    Ok(JsNumber::new(scope, num_cpus::get() as f64))
}

register_module!(m, {
    m.export("cpus", cpus)
});
