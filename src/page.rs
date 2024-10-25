use notion_client::objects::page::{Page, PageProperty};
use tracing::error;

pub trait PageExtended {
    fn get_reclaim_id(&self) -> String;

    fn validate(&self) -> bool;
}

static NOTION_KEY_REQUIREMENTS: [(&str); 10] = [
    "priority",
    "time_needed",
    "min_duration",
    "max_duration",
    "schedule_after",
    "due_date",
    "notes",
    "visibility",
    "reclaim_id",
    "snc_information",
];

impl PageExtended for Page {
    fn get_reclaim_id(&self) -> Option<String> {
        let tmp = self.properties.get("reclaim_id").unwrap();
        match tmp{
            PageProperty::Number { number, .. } => { Some(number.to_string()) }
            PageProperty::UniqueID { id,.. } => {
                Some(id.clone().unwrap())
            }
            _ => { None }
        }

    }

    fn validate(&self) -> bool {
        let properties = &self.properties;

        let property_keys = properties
            .keys()
            .clone()
            .into_iter()
            .map(|k| k.to_lowercase())
            .collect::<Vec<String>>();

        for i in NOTION_KEY_REQUIREMENTS {
            if !property_keys.contains(&i.to_lowercase()) {
                error!("Invalid notion page {}: Key {} is missing", self.id, i);
                return false;
            }
        }
        true
    }
}
