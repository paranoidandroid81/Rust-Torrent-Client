use crate::torrent_file::info_dict::{IStoreFileInfo, SingleFileInfo, MultiFileInfo};
use crate::torrent_file::torrent_file::TorrentFile;
use regex::Regex;
use std::error::Error;

pub struct TorrentFileParser
{

}

impl TorrentFileParser
{
    /*
    fn parse_file_to_object<T>(torrent_file: &str) -> TorrentFile<T> where T:IStoreFileInfo
    {
        if TorrentFileParser::is_multi_file(torrent_file)
        {
            TorrentFile::<MultiFileInfo>{}
        } else {
            TorrentFile::<SingleFileInfo>{}
        }
    }*/

    pub fn parse_announce(&self, raw_file: String) -> Result<String, String>
    {
        let announce_re = Regex::new(r"8:announce(\d+):(.*)");
        let announce_comp_re = match announce_re {
            Ok(re) => re,
            Err(e) => return Err(e.description().to_string())
        };
        let announce_re_match = match announce_comp_re.captures(&raw_file) {
            Some(announce_found) => announce_found,
            None => return Err("No match found for announce in torrent file!".to_string())
        };
        
        let announce_len_str = match announce_re_match.get(1) {
            Some(len) => len.as_str(),
            None => return Err("Unable to get length portion of announce string".to_string())
        };
        let announce_len = match announce_len_str.parse::<usize>() {
            Ok(len) => len,
            Err(e) => return Err(e.description().to_string())
        };
        let announce_remaining = match announce_re_match.get(2) {
            Some(remaining) => String::from(remaining.as_str()),
            None => return Err("Unable to get remaining portion of announce string".to_string())
        };
        Ok(String::from(&announce_remaining[..announce_len]))
    }

    pub fn is_valid_torrent_file(&self, torrent_file: &str) -> bool
    {
        let re = Regex::new(r"^d(.*)e$").unwrap();
        re.is_match(torrent_file)
    }

    pub fn is_multi_file(&self, torrent_file: &str) -> bool
    {
        torrent_file.contains("5:filesl")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_multi_file_correctly_identifies_multi_file_info()
    {
        let parser = TorrentFileParser{};
        assert_eq!(true, parser.is_multi_file("d4:name9:alice.txt5:filesld6:lengthi111e4:pathl7:111.txteed6:lengthi222e4:pathl7:222.txteeee"));
    }

    #[test]
    fn is_multi_file_correctly_return_false_single_file_info()
    {
        let parser = TorrentFileParser{};

        assert_eq!(false, parser.is_multi_file("d6:lengthi163783e4:name9:alice.txt12:piece lengthi16384e6:pieces11:ABCDEFGHIJKe"));
    }
    
    #[test]
    fn whole_file_matches_entire_bencoded_single_file()
    {
        let raw_string = "d6:lengthi163783e4:name9:alice.txt12:piece lengthi16384e6:pieces11:ABCDEFGHIJKe";
        let parser = TorrentFileParser{};
        assert_eq!(true, parser.is_valid_torrent_file(raw_string));
    }

    #[test]
    fn parse_announce_correctly_gets_announce_from_single_file()
    {
        let raw_string = "d8:announce40:udp://tracker.leechers-paradise.org:69696:lengthi163783e4:name9:alice.txt12:piece lengthi16384e6:pieces11:ABCDEFGHIJKe";
        let parser = TorrentFileParser{};
        let expected = "udp://tracker.leechers-paradise.org:6969";
        assert_eq!(expected, parser.parse_announce(String::from(raw_string)).unwrap());
    }
}