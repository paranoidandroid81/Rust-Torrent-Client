use rustc_serialize::Decoder;
use bencode::{encode, Decoder as OtherDecoder};
use std::collections::HashMap;
use rustc_serialize::{Encodable, Decodable};
use regex::Regex;


pub enum InfoDictLoadError
{
    FileInfoNotFound,
    UnsupportedInfoDictTypeFound
}

#[derive(Default, RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct InfoDict<T: IStoreFileInfo>
{
    piece_length: i32,
    pieces: String,
    file_info: T
}

#[derive(Default, RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct SingleFileInfo
{
    name: String,
    length: i32
}

impl SingleFileInfo
{
    fn new_empty() -> SingleFileInfo
    {
        SingleFileInfo{name: "".to_string(), length: 0} 
    }
}

#[derive(Default, RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct MultiFileInfo
{
    name: String,
    files: Vec<FilesDict>
}

impl MultiFileInfo
{
    fn new_empty() -> MultiFileInfo
    {
        MultiFileInfo{name: "".to_string(), files: vec!()} 
    }
}

impl InfoDict<SingleFileInfo>
{
    pub fn new_empty() -> InfoDict<SingleFileInfo>
    {
        InfoDict{piece_length: 0, pieces: "".to_string(), file_info: SingleFileInfo::new_empty()}
    }
}

impl InfoDict<MultiFileInfo>
{
    pub fn new_empty() -> InfoDict<MultiFileInfo>
    {
        InfoDict{piece_length: 0, pieces: "".to_string(), file_info: MultiFileInfo::new_empty()}
    }
}

#[derive(Default, RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct FilesDict
{
    length: i32,
    path: Vec<String>
}



impl IStoreFileInfo for SingleFileInfo 
{
}

impl IStoreFileInfo for MultiFileInfo
{

}

pub trait IStoreFileInfo
{
}

#[cfg(test)]
mod tests {
    use crate::torrent_file::file_parser::TorrentFileParser;
use super::*;

    

}
