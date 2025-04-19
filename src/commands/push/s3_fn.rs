use aws_sdk_s3::{Client, config::{Region, Credentials}, primitives::ByteStream};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::error::Error;

#[tokio::main]
pub async fn s3_fn(files: &Vec<(String, bool)>, repo_id: String) -> Result<(), Box<dyn Error>> {
    for file in files {

        let file_path = file.0.trim();
    
        // Parse file name
        let path = Path::new(file_path);
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                eprintln!("Invalid file path.");
                return Ok(());
            }
        };
    
        // Read file content
        let mut file = BufReader::new(File::open(path)?);
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
    
        // Set your hardcoded credentials and region
        let credentials = Credentials::new(
            "AKIAR3HUOQWVQJDYVLAV",
            "KeHlgE+WWPyla1mMqddjiAzCgfMhllbNDfLbuom1",     // 🔐 your secret key
            None,
            None,
            "static",
        );
    
        let config = aws_sdk_s3::config::Builder::new()
            .credentials_provider(credentials)
            .region(Region::new("ap-south-1")) // or your desired region
            .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest()) // ✅ REQUIRED
            .build();
    
        let client = Client::from_conf(config);
    
        // Set your bucket name
        let bucket_name = "dhairyasingla-delta";
    
        // Upload to S3
        let body = ByteStream::from(buffer.clone());
        
        let file_path = format!("{}{}", repo_id, &file_name[1..]); 

        client
            .put_object()
            .bucket(bucket_name)
            .key(&file_path)
            .body(body)
            .send()
            .await?;

        println!("{} uploaded successfully !!!", &file_path);
    }


    Ok(())
}