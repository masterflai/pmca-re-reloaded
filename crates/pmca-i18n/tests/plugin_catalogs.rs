//! Plugin catalogues are a process-wide registry, so this lives in its own test
//! binary: registering here cannot leak into the crate's other tests.

use pmca_i18n::{Localizer, catalog, register_plugin_catalogs};

#[test]
fn plugin_catalogues_add_languages_and_messages_but_never_reword_the_program() {
    assert!(register_plugin_catalogs(vec![
        // A language the program does not ship, as a full language pack.
        ("eo".to_string(), "app-title = Esperanto-Titolo\nplg-demo-hello = Saluton\n".to_string()),
        // A shipped language: the plugin's own message plus an attempt to
        // reword a program message, which must not take effect.
        ("de-DE".to_string(), "plg-demo-hello = Hallo\napp-title = Gekapert\n".to_string()),
        // A second plugin defining the same id: the first one wins.
        ("de-DE".to_string(), "plg-demo-hello = Zweiter\n".to_string()),
    ]));
    assert!(!register_plugin_catalogs(Vec::new()), "registration is once per process");

    let eo = Localizer::new("eo");
    assert_eq!(eo.current(), "eo");
    assert!(eo.available().contains(&"eo".to_string()));
    assert_eq!(eo.tr("plg-demo-hello"), "Saluton");
    assert_eq!(eo.tr("app-title"), "Esperanto-Titolo");
    // Anything the pack leaves out falls back to en-US rather than to the id.
    assert_ne!(eo.tr("btn-cancel"), "btn-cancel");

    let de = Localizer::new("de-DE");
    assert_eq!(de.tr("plg-demo-hello"), "Hallo");
    let shipped_title = Localizer::new("en-US");
    assert_ne!(de.tr("app-title"), "Gekapert");
    assert_ne!(shipped_title.tr("app-title"), "Gekapert");

    // The window parses the served catalogue line by line, last line winning;
    // the served text must therefore resolve the same way the backend does.
    let served = catalog::effective_source("de-DE").expect("catalogue");
    assert!(served.contains("plg-demo-hello = Hallo"));
    assert!(!served.contains("Gekapert"));
    assert!(!served.contains("plg-demo-hello = Zweiter"));

    let served_eo = catalog::effective_source("eo").expect("plugin-only language is served");
    assert!(served_eo.contains("plg-demo-hello = Saluton"));
    assert!(catalog::effective_source("xx-XX").is_none());
}
