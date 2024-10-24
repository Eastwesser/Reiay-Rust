//  Этот файл отвечает за работу с переводами. Здесь загружаются файлы переводов и происходит обработка мультиязычности.

use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
use std::collections::HashMap;
use std::sync::RwLock;
use unic_langid::langid;

lazy_static::lazy_static! {
    static ref FLUENT_BUNDLES: RwLock<HashMap<String, FluentBundle<FluentResource>>> = RwLock::new(HashMap::new());
}

pub fn init_fluent() {
    let en_translation = include_str!("translations/en.ftl");
    let ru_translation = include_str!("translations/ru.ftl");

    let en_resource = FluentResource::try_new(en_translation.to_string()).unwrap();
    let ru_resource = FluentResource::try_new(ru_translation.to_string()).unwrap();

    let mut en_bundle = FluentBundle::new_concurrent(vec![langid!("en")]);
    let mut ru_bundle = FluentBundle::new_concurrent(vec![langid!("ru")]);

    en_bundle.add_resource(en_resource).unwrap();
    ru_bundle.add_resource(ru_resource).unwrap();

    let mut bundles = FLUENT_BUNDLES.write().unwrap();
    bundles.insert("en".to_string(), en_bundle);
    bundles.insert("ru".to_string(), ru_bundle);
}

pub fn translate(lang: &str, key: &str) -> String {
    let bundles = FLUENT_BUNDLES.read().unwrap();
    let bundle = bundles.get(lang).unwrap_or_else(|| bundles.get("en").unwrap());

    let mut errors = vec![];
    let value = bundle
        .format_pattern(bundle.get_message(key).unwrap().value().unwrap(), None, &mut errors);

    value.to_string()
}
