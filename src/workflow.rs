use serde::{Serialize, Deserialize};
use uuid::Uuid;
use random::RandomKind;
use crate::{GenerateRandom, random};

static AUTHOR_WEBSITE: &str = "https://hongdenglv.com";

#[derive(Serialize, Deserialize)]
pub struct Workflows {
    pub items: Vec<Workflow>,
}




#[derive(Serialize, Deserialize)]
pub struct Workflow {
    arg: String,
    quicklookurl: String,
    subtitle: String,
    title: String,
    uid: String,
}
#[derive(Serialize, Deserialize)]
pub struct Text {
    copy: String,
    largetype: String,
}

impl Workflows {
    pub fn new(generate_value: Vec<GenerateRandom>) -> Workflows {
        let mut items = Vec::new();
        for value in generate_value {
            let workflow = Workflow::new(value.value.clone(), value.kind);
            items.push(workflow);
        }
        Workflows {
            items,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Workflow {
    pub fn new(arg: String, kind: RandomKind) -> Workflow {
        Workflow {
            arg: arg.clone(),
            quicklookurl: AUTHOR_WEBSITE.to_string(),
            subtitle: kind.get_name(),
            title: arg.clone(),
            uid: String::from(Uuid::new_v4()),
        }
    }
}