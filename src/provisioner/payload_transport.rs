use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub fn deploy_payload(payload: &str, config: &Yaml) {
    let loaded_from_payload = YamlLoader::load_from_str(payload).expect("Failed out to load yaml from payload");
    let payload_config = loaded_from_payload.into_iter()
        .next()
        .expect("payload is empty");

    println!("{} {:?}", config["kind"].as_str().unwrap_or(""), payload_config["kind"].as_str().unwrap_or(""));
}
