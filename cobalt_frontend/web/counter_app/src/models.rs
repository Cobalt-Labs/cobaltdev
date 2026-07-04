use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameReq {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameResp {
    pub msg: String,
    pub stored_name: String, 
}