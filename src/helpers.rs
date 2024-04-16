use rocket::request::FlashMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FlashLabel<'a> {
    pub alert_type: &'static str,
    pub content: &'a str,
}

impl<'a> FlashLabel<'a> {
    pub fn success(message: &'a str) -> FlashLabel {
        FlashLabel {
            alert_type: "success",
            content: message,
        }
    }

    pub fn error(message: &'a str) -> FlashLabel {
        FlashLabel {
            alert_type: "error",
            content: message,
        }
    }

    pub fn warning(message: &'a str) -> FlashLabel {
        FlashLabel {
            alert_type: "warning",
            content: message,
        }
    }

    pub fn default(message: &'a str) -> FlashLabel {
        FlashLabel {
            alert_type: "default",
            content: message,
        }
    }

    pub fn none() -> FlashLabel<'a> {
        FlashLabel {
            alert_type: "none",
            content: "",
        }
    }
}

pub fn flash_label<'a>(flash: &'a Option<FlashMessage>) -> FlashLabel<'a> {
    if let Some(fl) = flash {
        match fl.kind() {
            "success" => FlashLabel::success(fl.message()),
            "error" => FlashLabel::error(fl.message()),
            "warning" => FlashLabel::warning(fl.message()),
            _ => FlashLabel::default(fl.message()),
        }
    } else {
        FlashLabel::none()
    }
}
