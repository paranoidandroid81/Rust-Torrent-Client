use crate::torrent_file::info_dict::{SingleFileInfo, MultiFileInfo};
use crate::torrent_file::info_dict::
    {InfoDict, IStoreFileInfo};


pub enum TorrentFileLoadError
{
    InfoDictNotFound,
    UnsupportedTypeFound
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct TorrentFile<T: IStoreFileInfo>
{
    info: InfoDict<T>,
    announce: String
}

impl TorrentFile<SingleFileInfo> 
{
    pub fn new_empty() -> TorrentFile<SingleFileInfo>
    {
        TorrentFile::<SingleFileInfo>{announce: "".to_string(), info: InfoDict::<SingleFileInfo>::new_empty()}
    }
    
}


impl TorrentFile<MultiFileInfo>
{
    pub fn new_empty() -> TorrentFile<MultiFileInfo>
    {
        TorrentFile::<MultiFileInfo>{announce: "".to_string(), info: InfoDict::<MultiFileInfo>::new_empty()}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
}