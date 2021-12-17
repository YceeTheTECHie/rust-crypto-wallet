use anyhow::Result;
mod wallets;
use web3::types::{Address};
use std::{  str::FromStr};
use tokio;
const URL : &str = "https://eth-ropsten.alchemyapi.io/v2/IKep9GN_OVBd_6JG5LCTtN5TZ5irvpeU";

#[tokio::main]

async fn main() -> Result<()> {
    let keypair = wallets::create_keypair()?;
    println!("{:?}", keypair);
    let web3 = wallets::establish_connection(URL)?;
    let to_address = Address::from_str("0x1b42ee3294DA8A1Fa125061674067128e2AFEB6c")?;
    let tx_object = wallets::create_tx_object(to_address,7)?;
    let result = wallets::sign_tx(web3.clone(),tx_object,keypair.0).await?;
    println!("result {:?}", result);
    Ok(())
}
