pub const SERVER_ADDRESS: &str = "http://77.75.125.164:8000";

pub fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

pub fn name_to_minecraft_id(name: String) -> String {
    format!("minecraft:{}", name.replace(" ", "_").to_lowercase())
}
