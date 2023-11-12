pub fn escape_message(msg: &str) -> String {
    msg.replace('.', "\\.")
}
