use crate::error::Result;

pub async fn send_job(name: &String) -> Result<()> {
    println!("Sending job: {} to Google pub/sub", name);
    Ok(())
}
