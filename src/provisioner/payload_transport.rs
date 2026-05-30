use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use crate::provisioner::network_transport::ssh_run;

pub fn deploy_payload(payload: &str, config: &Yaml) {
    let loaded_from_payload = YamlLoader::load_from_str(payload).expect("Failed out to load yaml from payload");
    let payload_config = loaded_from_payload.into_iter()
        .next()
        .expect("payload is empty");

    
    check_required_values(&config);
        
    ssh_run(
        &config["ssh_user"].as_str().unwrap_or(""), 
        &config["remote_host"].as_str().unwrap_or(""),
        "cd ~",
        &config["identity_file"].as_str().unwrap_or("")
    );

    println!("{} {:?}", config["kind"].as_str().unwrap_or(""), payload_config["kind"].as_str().unwrap_or(""));
}

fn check_required_values(cfg: &Yaml) {
    cfg["ssh_user"]
        .as_str()
        .expect("Missing or invalid 'ssh_user' in torso.yaml");

    cfg["remote_host"]
        .as_str()
        .expect("Missing or invalid 'remote_host' in torso.yaml");

    cfg["identity_file"]
        .as_str()
        .expect("Missing or invalid 'identity_file' in torso.yaml");
}
