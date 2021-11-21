use anyhow::Result;
mod wallets;

fn main() -> Result<()> {
    let keypair = wallets::create_keypair();
    println!("{:?}", keypair);
    Ok(())
}
