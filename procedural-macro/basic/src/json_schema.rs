use serde::Deserialize;
use std::collections::{ HashMap, Vec };
use heck::{ AsPascalCase, AsSnakeCase };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Schema {
    title: String,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
    description: Option<String>
}

#[derive(Debug)]
pub struct St {
    name: String,
    fields: Vec<Fd>
}

#[derive(Debug)]
pub struct Fd {
    name: String,
    ty: String
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self  {
            name: name.into(),
            fields
        }
    } 
}
impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name,
            ty
        }
    }
}

impl Schema {
    pub fn into_st(&self) -> Vec<St> {
        let mut structs = vec![];
        match self.ty {
            "object" => {
                let fields = self.properties.as_ref().unwrap()
                    .iter()
                    .map(|(k, v)| process_type(k.as_str(), v))
                    .collect();
                structs.push(St::new(p(self.title.as_ref().unwrap(), fields)));
            },
            _ => panic!("Not supported yet!")
        }
    }
}

fn process_type(k: &str, v: &Schema) -> Fd {
    match v.ty.as_str() {
        "object" => {
            todo!()
        },
        "integer" => {
            Fd::new(n(k), "i64")
        },
        "float" => {
            Fd::new(n(k), "f64")
        },
        "String" => {
            Fd::new(n(k), "String")
        }
        v => panic!("Unsupported type: {}", v)
    }
}

fn p(s: &str) -> String {
    todo!()
}

fn n(s: &str) -> Into<String> {
    p(s).into()
}
