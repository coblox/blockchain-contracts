use crate::calculate_offsets::{
    calc_offset,
    ethereum::compile_contract::compile,
    metadata::Metadata,
    placeholder_config::{Placeholder, PlaceholderConfig},
    Contract,
};
use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use std::convert::TryFrom;
use std::path::Path;

pub struct EthereumContract {
    bytes: Vec<u8>,
    placeholder_config: PlaceholderConfig,
}

impl EthereumContract {
    fn replace_contract_offset_parameters_in_header(header: &mut [u8], body: &[u8]) -> Result<()> {
        let body_length = body.len();
        let header_length = header.len();

        Self::replace_offset_parameter_in_header(
            "1001",
            "start of contract when loading into memory",
            header_length,
            header,
        )?;
        Self::replace_offset_parameter_in_header(
            "2002",
            "end of contract when loading into memory",
            body_length,
            header,
        )?;
        Self::replace_offset_parameter_in_header(
            "3003",
            "length of contract when returning for execution",
            body_length,
            header,
        )?;

        Ok(())
    }

    fn replace_offset_parameter_in_header(
        replace_pattern: &str,
        name: &str,
        value: usize,
        header: &mut [u8],
    ) -> Result<()> {
        let header_placeholder = Placeholder {
            name: name.into(),
            replace_pattern: replace_pattern.into(),
        };

        let header_placeholder_offset = calc_offset(&header_placeholder, header)?;

        let header_slice =
            &mut header[header_placeholder_offset.start..header_placeholder_offset.excluded_end];

        BigEndian::write_u16(header_slice, u16::try_from(value)?);

        Ok(())
    }
}

impl Contract for EthereumContract {
    fn compile<S: AsRef<Path>>(template_folder: S) -> Result<EthereumContract> {
        let mut bytes = compile(template_folder.as_ref().join("deploy_header.asm"))?;
        let mut contract_body = compile(template_folder.as_ref().join("contract.asm"))?;

        Self::replace_contract_offset_parameters_in_header(&mut bytes, &contract_body)?;

        bytes.append(&mut contract_body);

        let placeholder_config =
            PlaceholderConfig::from_file(template_folder.as_ref().join("config.json"))?;

        Ok(Self {
            bytes,
            placeholder_config,
        })
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            protocol_name: self.placeholder_config.protocol_name.clone(),
            contract: self.bytes.to_owned(),
        }
    }

    fn placeholder_config(&self) -> &PlaceholderConfig {
        &self.placeholder_config
    }

    fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
