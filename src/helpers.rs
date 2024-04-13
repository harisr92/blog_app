use rocket::request::FlashMessage;

pub fn flash_label(flash: Option<FlashMessage>) -> String {
    if let Some(fl) = flash {
        match fl.kind() {
            "success" => String::from(format!(
                "<label class=\"flash-message success\">{}</label>",
                fl.message()
            )),
            "error" => String::from(format!(
                "<label class=\"flash-message danger\">{}</label>",
                fl.message()
            )),
            "warning" => String::from(format!(
                "<label class=\"flash-message warn\">{}</label>",
                fl.message()
            )),
            _ => String::from(format!(
                "<label class=\"flash-message\">{}</label>",
                fl.message()
            )),
        }
    } else {
        String::from("")
    }
}
