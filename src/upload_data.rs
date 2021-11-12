use std::sync::{Arc, Mutex};

use async_recursion::async_recursion;
use reqwest::{Client, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};

// The innermost struct of this script, here for easy serialization of the "custom_fields" required
// for each Request's JSON body.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "PascalCase"))]
pub struct CustomFields {
    pub print: String,
    pub electrical: String,
    pub workshop: String,
    pub foliage: String,
    pub upholstery: String,
    pub warehouse: String
}

// The first inner struct of the UploadMaster. Allows the serde serializer to build the "product"
// level of the the JSON body, and contains the product_id and "custom_fields" inner levels.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "PascalCase"))]
pub struct UploadRecord {
    pub id: usize,
    pub custom_fields: CustomFields
}

// The highest level struct. To be called, in order to 
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "PascalCase"))]
pub struct UploadMaster<'a> {
    #[serde(skip)] 
    client: &'a Client,
    #[serde(skip)]
    wtr: &'a Arc<Mutex<csv::Writer<std::fs::File>>>,
    pub product: UploadRecord,
}

// The core struct of the data which is to be uploaded. An UploadMaster represents an individual
// product (UploadRecord), which in turn is comprised of an id and a series of custom
// field values (CustomFields).
impl<'a> UploadMaster<'a> {
    // Takes one argument, a product (UploadRecord), and returns a new UploadMaster struct, which
    // is used as the core for sending requests to the Current RMS REST API.
    pub fn new(client: &'a Client, 
        wtr: &'a Arc<Mutex<csv::Writer<std::fs::File>>>,
        product: UploadRecord) -> UploadMaster<'a> {
        UploadMaster {
            client,
            wtr,
            product,
        }
    }

    #[async_recursion]
    pub async fn send(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url: Url = self.build_url().await?;
        println!("Building PUT request for product ID: {}", &self.product.id);
        let response: Response = self.client.put(url).send().await?;
        self.confirm_status(response).await?;
        Ok(())
    }

    // Takes the product ID from the UploadMaster's product, and returns a target URL for updating
    // the current product.
    pub async fn build_url(&self) -> Result<Url, Box<dyn std::error::Error>> {
        let url: String = format!("https://api.current-rms.com/api/v1/products/{}",
                                &self.product.id);
        Ok(url.parse()?)
    }

    // Checks the status code of the returned response. Returns Ok(()) if the status code is 200,
    // makes the thread sleep in the event of a 429 status code, or saves to 'failed_uploads.csv'
    // in the event of any other status code.
    async fn confirm_status(&self, res: Response) -> Result<(), 
                                                Box<dyn std::error::Error>> {
        match res.status() {
            StatusCode::OK => self.is_ok().await, 
            StatusCode::TOO_MANY_REQUESTS => self.limit_reached(res).await,
            _ => self.all_other_responses().await,
        }
    }

    // Takes one argument, the response, and reads the 'X-RateLimit-Reset' header. Passes this as a
    // String to the time::sleep() method, to make the thread sleep until the rate limit reset.
    async fn limit_reached(&self, res: Response) -> Result<(), 
                                                Box<dyn std::error::Error>> {
        
        let response_headers = res.headers();
        let limit_reset_header: &str = response_headers.get("X-RateLimit-Reset").unwrap().to_str()?;
        crate::response_handlers::sleep(limit_reset_header)?;

        self.send().await?;
        Ok(())
    }

    async fn all_other_responses(&self) -> Result<(), Box<dyn std::error::Error>> {
        let writer = Arc::clone(&self.wtr);
        let mut wtr_ref = writer.lock().unwrap();

        wtr_ref.serialize(&self)?;
        Ok(())
    }

    // Here to simply return okay, 
    async fn is_ok(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}