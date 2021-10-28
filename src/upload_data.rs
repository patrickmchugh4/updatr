use reqwest::{Client, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "PascalCase"))]
pub struct UploadMaster {
    pub product: UploadRecord
}

impl UploadMaster {
    pub fn new(product: UploadRecord) -> UploadMaster {
        UploadMaster {
            product
        }
    }

    // Send the put request. Ignore if request is Ok. Store request and 
    // resend if limit reached.
    pub async fn send(&self, client: &Client) -> Result<(), 
                                                    Box<dyn std::error::Error>> {
        let url: Url = self.build_url().await?;
        let res: Response = client.put(url).json(&self).send().await?;

        Ok(())
    }

    async fn build_url(&self) -> Result<Url, Box<dyn std::error::Error>> {
        let url: String = format!("https://api.current-rms.com/api/v1/products/{}",
                                &self.product.id);
        Ok(url.parse()?)
    }

    async fn check_status(&self, res: Response) -> Result<(), 
                                                Box<dyn std::error::Error>> {
        match res.status() {
            StatusCode::OK => println!("ID: {} Updated Succesfully.", 
                                        self.product.id),
            StatusCode::TOO_MANY_REQUESTS => self.limit_reached(res).await?,
            _ => self.unhandled_response()?,
        }

        Ok(())
    }

    async fn limit_reached(&self, res: Response) -> Result<(), 
                                                Box<dyn std::error::Error>> {
        Ok(())
    }

    fn unhandled_response(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "PascalCase"))]
pub struct UploadRecord {
    pub id: usize,
    pub custom_fields: CustomFields
}

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