use std::env;
use std::ffi::OsString;
use std::sync::{Arc, Mutex};

use csv;
use reqwest::Client;

mod errors;
mod client;
mod response_handlers;
mod upload_data;
use upload_data::{UploadMaster, UploadRecord};

fn get_file_path() -> Result<OsString, Box<dyn std::error::Error>> {
    match env::args_os().nth(1) {
        None => Err(errors::NoFilePath.into()),
        Some(file_path) => Ok(file_path)
    }
}

fn reader(file_path: OsString) -> Result<csv::Reader<std::fs::File>,
                                            Box<dyn std::error::Error>> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    Ok(rdr)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = client::build_client();

    let file_path: OsString = get_file_path()?;
    let mut rdr: csv::Reader<std::fs::File> = reader(file_path)?;
    let mut _wtr = Arc::new(Mutex::new(csv::Writer::from_path("failed_uploads.csv")?));

    for result in rdr.deserialize() {
        let record: UploadRecord = result?;
        let master_record: UploadMaster = UploadMaster::new(&client, &_wtr, record);
        master_record.send().await?;
    }

    Ok(())
}