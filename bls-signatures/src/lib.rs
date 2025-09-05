#![no_std]
#![cfg_attr(feature = "frozen-abi", feature(min_specialization))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;
#[cfg(not(any(target_os = "solana", target_os = "zkvm")))]
pub use crate::{
    error::BlsError,
    keypair::Keypair,
    proof_of_possession::{
        AsProofOfPossessionProjective, ProofOfPossessionProjective, VerifiableProofOfPossession,
    },
    pubkey::{AsPubkeyProjective, PubkeyProjective, VerifiablePubkey},
    secret_key::{SecretKey, BLS_SECRET_KEY_SIZE},
    signature::{AsSignatureProjective, SignatureProjective, VerifiableSignature},
};
pub use crate::{
    proof_of_possession::{
        ProofOfPossession, ProofOfPossessionCompressed, BLS_PROOF_OF_POSSESSION_AFFINE_SIZE,
        BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE,
    },
    pubkey::{
        Pubkey, PubkeyCompressed, BLS_PUBLIC_KEY_AFFINE_SIZE, BLS_PUBLIC_KEY_COMPRESSED_SIZE,
    },
    signature::{
        Signature, SignatureCompressed, BLS_SIGNATURE_AFFINE_SIZE, BLS_SIGNATURE_COMPRESSED_SIZE,
    },
};

pub mod error;
#[cfg(not(any(target_os = "solana", target_os = "zkvm")))]
pub mod keypair;
#[macro_use]
pub(crate) mod macros;
#[cfg(not(any(target_os = "solana", target_os = "zkvm")))]
pub mod hash;
pub mod proof_of_possession;
pub mod pubkey;
#[cfg(not(any(target_os = "solana", target_os = "zkvm")))]
pub mod secret_key;
pub mod signature;
