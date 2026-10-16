//! Plugin settings and switches are process-wide registries, so this lives in
//! its own test binary: registering here cannot leak into the unit tests.

use std::collections::BTreeMap;

use pmca_platform::props::{self, PropDef};
use pmca_platform::tweaks::{self, PluginTweak, Presentation};
use pmca_platform::{DirectPatch, Settings};

const PROP_ID: u32 = 0x0123_0456;
const MODEL_NAME_ID: u32 = 0x003e_0005;

fn camera(model: &str, value: u8) -> DirectPatch {
    let mut name = model.as_bytes().to_vec();
    name.resize(16, 0);
    let values = BTreeMap::from([(MODEL_NAME_ID, name), (PROP_ID, vec![value])]);
    DirectPatch::new("EU_CE3", values)
}

#[test]
fn a_plugin_switch_obeys_the_same_guard_as_the_built_in_ones() {
    assert!(props::register_extra(vec![PropDef {
        name: "plg-demo-flag",
        base: PROP_ID,
        count: 1,
        part_size: 1,
    }]));
    assert!(tweaks::register_plugin_tweaks(vec![PluginTweak {
        id: "plg-demo-flag",
        prop: "plg-demo-flag",
        off: vec![0x10],
        on: vec![0x20],
        models: vec!["ILCE-5100".to_string()],
    }]));
    assert!(props::find("plg-demo-flag").is_some());
    assert_eq!(tweaks::presentation_of("plg-demo-flag").unwrap(), Presentation::Toggle);

    // Right model, value is the plugin's "off": offered and switchable.
    let mut cam = camera("ILCE-5100", 0x10);
    let status = tweaks::list(&cam).into_iter().find(|t| t.id == "plg-demo-flag").unwrap();
    assert!(!status.enabled);
    tweaks::set_enabled(&mut cam, "plg-demo-flag", true, false).unwrap();
    assert_eq!(cam.read("plg-demo-flag").unwrap(), vec![0x20]);
    assert!(tweaks::list(&cam).iter().any(|t| t.id == "plg-demo-flag" && t.enabled));

    // A value the plugin did not name: never offered, never written.
    let mut odd = camera("ILCE-5100", 0x33);
    assert!(tweaks::list(&odd).iter().all(|t| t.id != "plg-demo-flag"));
    assert!(tweaks::set_enabled(&mut odd, "plg-demo-flag", true, false).is_err());

    // Another model: not offered even though the value would match.
    let mut other = camera("ILCE-7M3", 0x10);
    assert!(tweaks::list(&other).iter().all(|t| t.id != "plg-demo-flag"));
    assert!(tweaks::set_enabled(&mut other, "plg-demo-flag", true, false).is_err());

    // The direct path reads plugin settings like built-in ones.
    assert!(DirectPatch::wanted_ids().any(|id| id == PROP_ID));
}
