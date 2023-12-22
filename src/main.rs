mod connection;
mod version;

use version::Version;

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let version = Version::new().await?;

    println!("{:#?}", version);

    Ok(())
}
