use std::io::{Read, Seek};

use anyhow::{anyhow, Result};
use zip::{read::ZipFile, ZipArchive};

use crate::{read_timeline_records_from_json, TimelineRecordsIter};

pub fn read_timeline_records_from_archive<'a, A: Read + Seek>(
    archive: &mut ZipArchive<A>,
) -> Result<TimelineRecordsIter<ZipFile>> {
    let file_path = archive
        .file_names()
        .find(|name| name.ends_with("Records.json"))
        .ok_or(anyhow!("No timeline Records.json file found in archive"))?
        .to_string();

    let file = archive.by_name(&file_path)?;

    read_timeline_records_from_json(file)
}
