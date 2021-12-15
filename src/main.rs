use anyhow::Result;
mod wallets;

const URL : &str = "https://eth-ropsten.alchemyapi.io/v2/IKep9GN_OVBd_6JG5LCTtN5TZ5irvpeU";
fn main() -> Result<()> {
    let keypair = wallets::create_keypair();
    println!("{:?}", keypair);
    let web3 = wallets::establish_connection(URL)?;
    Ok(())
}
