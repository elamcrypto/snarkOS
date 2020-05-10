use crate::{AccountPrivateKey, AccountPublicKey};
use snarkos_errors::objects::AccountError;
use snarkos_models::{dpc::DPCComponents, objects::AccountScheme};
use snarkos_utilities::bytes::{FromBytes, ToBytes};

use rand::Rng;
use std::io::{Read, Result as IoResult, Write};

#[derive(Derivative)]
#[derivative(Clone(bound = "C: DPCComponents"))]
pub struct Account<C: DPCComponents> {
    pub public_key: AccountPublicKey<C>,
    pub private_key: AccountPrivateKey<C>,
    pub is_testnet: bool,
}

impl<C: DPCComponents> AccountScheme for Account<C> {
    type AccountPrivateKey = AccountPrivateKey<C>;
    type AccountPublicKey = AccountPublicKey<C>;
    type CommitmentScheme = C::AddressCommitment;
    type SignatureScheme = C::Signature;

    /// Creates a new account. Defaults to a testnet account
    /// if no network indicator is provided.
    fn new<R: Rng>(
        signature_parameters: &Self::SignatureScheme,
        commitment_parameters: &Self::CommitmentScheme,
        metadata: &[u8; 32],
        is_testnet: Option<bool>,
        rng: &mut R,
    ) -> Result<Self, AccountError> {
        let private_key = AccountPrivateKey::new(signature_parameters, metadata, is_testnet, rng)?;
        let public_key = AccountPublicKey::from(commitment_parameters, &private_key)?;
        let is_testnet = private_key.is_testnet;

        Ok(Self {
            private_key,
            public_key,
            is_testnet,
        })
    }
}

impl<C: DPCComponents> ToBytes for Account<C> {
    fn write<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.public_key.write(&mut writer)?;
        self.private_key.write(&mut writer)?;
        self.is_testnet.write(&mut writer)
    }
}

impl<C: DPCComponents> FromBytes for Account<C> {
    /// Reads in an account buffer. Defaults to a testnet account
    /// if no network indicator is provided.
    #[inline]
    fn read<R: Read>(mut reader: R) -> IoResult<Self> {
        let public_key: AccountPublicKey<C> = FromBytes::read(&mut reader)?;
        let private_key: AccountPrivateKey<C> = FromBytes::read(&mut reader)?;
        let is_testnet: bool = match FromBytes::read(&mut reader) {
            Ok(is_testnet) => is_testnet,
            _ => true, // Defaults to testnet
        };

        Ok(Self {
            private_key,
            public_key,
            is_testnet,
        })
    }
}
