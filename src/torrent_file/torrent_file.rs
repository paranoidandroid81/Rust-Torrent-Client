use crate::torrent_file::info_dict::
    {Node, Info};
use serde_bencode::de;
use serde_derive::{Serialize, Deserialize};
use serde_bytes::ByteBuf;
use std::io::{self, Read};
   
#[derive(Debug, Deserialize)]
pub struct Torrent {
    info: Info,
    announce: String,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}