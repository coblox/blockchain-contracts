#![warn(
    unused_extern_crates,
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::fallible_impl_from,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::print_stdout,
    clippy::dbg_macro
)]
#![forbid(unsafe_code)]

mod calculate_offsets;

use self::calculate_offsets::{
    bitcoin::BitcoinScript, ethereum::contract::EthereumContract, offset::to_markdown, Contract,
};
use crate::calculate_offsets::placeholder_offsets;
use std::ffi::OsStr;

const ETHER_TEMPLATE_FOLDER: &str = "./src/bin/calculate_offsets/ethereum/templates/ether/";
const ERC20_TEMPLATE_FOLDER: &str = "./src/bin/calculate_offsets/ethereum/templates/erc20/";
const BITCOIN_TEMPLATE_FOLDER: &str = "./src/bin/calculate_offsets/bitcoin/templates/bitcoin/";

#[allow(clippy::print_stdout)]
fn main() -> Result<(), Error> {
    println!("### RFC003 ###");

    println!(
        "{}",
        generate_markdown::<BitcoinScript, &str>(BITCOIN_TEMPLATE_FOLDER)?
    );
    println!(
        "{}",
        generate_markdown::<EthereumContract, &str>(ETHER_TEMPLATE_FOLDER)?
    );
    println!(
        "{}",
        generate_markdown::<EthereumContract, &str>(ERC20_TEMPLATE_FOLDER)?
    );

    Ok(())
}

fn generate_markdown<C: Contract, S: AsRef<OsStr>>(template_folder: S) -> Result<String, C::Error> {
    let contract = C::compile(template_folder)?;

    let metadata = contract.metadata();
    let offsets = placeholder_offsets(contract)?;

    Ok(format!(
        "{}\n{}",
        metadata.to_markdown(),
        to_markdown(offsets)
    ))
}

#[derive(Debug)]
enum Error {
    BitcoinScript(self::calculate_offsets::bitcoin::Error),
    EthereumContract(self::calculate_offsets::ethereum::Error),
}

impl From<self::calculate_offsets::bitcoin::Error> for Error {
    fn from(err: self::calculate_offsets::bitcoin::Error) -> Self {
        Error::BitcoinScript(err)
    }
}

impl From<self::calculate_offsets::ethereum::Error> for Error {
    fn from(err: self::calculate_offsets::ethereum::Error) -> Self {
        Error::EthereumContract(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain_contracts::ethereum::{erc20_htlc, ether_htlc};

    #[test]
    fn ether_contract_template_matches_template_in_calculate_offsets() -> Result<(), Error> {
        let contract = EthereumContract::compile(ETHER_TEMPLATE_FOLDER)?;
        assert_eq!(
            ether_htlc::CONTRACT_TEMPLATE.to_vec(),
            contract.metadata().contract,
        );
        Ok(())
    }

    #[test]
    fn erc20_contract_template_matches_template_in_calculate_offsets() -> Result<(), Error> {
        let contract = EthereumContract::compile(ERC20_TEMPLATE_FOLDER)?;
        assert_eq!(
            erc20_htlc::CONTRACT_TEMPLATE.to_vec(),
            contract.metadata().contract,
        );
        Ok(())
    }
}
