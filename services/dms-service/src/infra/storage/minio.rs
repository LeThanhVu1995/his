use s3::{creds::Credentials, Bucket, Region};

#[derive(Clone)]
pub struct S3{
  pub bucket: Bucket
}

impl S3{
  pub async fn from_env()->anyhow::Result<Self>{
    let endpoint=std::env::var("S3_ENDPOINT")?;
    let region=Region::Custom{
      region: std::env::var("S3_REGION").unwrap_or("us-east-1".into()),
      endpoint: endpoint.clone()
    };
    let creds=Credentials::new(
      Some(&std::env::var("S3_ACCESS_KEY")?),
      Some(&std::env::var("S3_SECRET_KEY")?),
      None, None, None
    )?;
    let mut bucket=Bucket::new(&std::env::var("S3_BUCKET")?, region, creds)?;
    if std::env::var("S3_USE_PATH_STYLE").unwrap_or("true".into())=="true" {
      bucket.set_path_style();
    }
    Ok(Self{ bucket })
  }

  pub async fn put(&self, key:&str, bytes:&[u8], content_type:&str)->anyhow::Result<()> {
    let response = self.bucket.put_object_with_content_type(key, bytes, content_type).await?;
    if response.status_code()/100 != 2 {
      anyhow::bail!("s3 put error: {}", response.status_code());
    }
    Ok(())
  }

  pub async fn presign_put(&self, key:&str, expires:u32, content_type:&str)->anyhow::Result<String>{
    Ok(self.bucket.presign_put(key, expires, None).await?)
  }

  pub async fn presign_get(&self, key:&str, expires:u32)->anyhow::Result<String>{
    Ok(self.bucket.presign_get(key, expires, None).await?)
  }
}
