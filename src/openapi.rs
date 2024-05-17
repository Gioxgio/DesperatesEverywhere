use apistos::{app::BuildConfig, info::Info, spec::Spec, SwaggerUIConfig};

pub fn get_documentation() -> Spec {
    Spec {
        info: Info {
            title: "Desperates everywhere".to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn get_json_path() -> String {
    "/openapi.json".to_string()
}

pub fn get_swagger_configuration() -> BuildConfig {
    BuildConfig::default().with(SwaggerUIConfig::new(&"/swagger"))
}
