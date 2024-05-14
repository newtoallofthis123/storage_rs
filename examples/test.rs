use storage_client::{bucket::BucketCreateOptions, client::StorageClient, errors::StorageError};

#[tokio::main()]
async fn main() -> Result<(), StorageError> {
    let sc = StorageClient::new("https://uitdftmoidztlizibgvn.supabase.co/storage/v1/", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InVpdGRmdG1vaWR6dGxpemliZ3ZuIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTcxNTY2NjMwNCwiZXhwIjoyMDMxMjQyMzA0fQ.HDiKKeNatn3lHbg0s7MWPslbJMrCkF7Z9QoR9khKiF4").unwrap();

    println!("{:?}", sc.create_bucket(BucketCreateOptions::default("test", true)).await?);
    Ok(())
}
