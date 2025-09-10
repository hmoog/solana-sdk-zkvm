#![no_std]
#[cfg(all(feature = "sha2", not(any(target_os = "solana", target_os = "zkvm"))))]
use sha2::{Digest, Sha256};
use solana_hash::Hash;

#[cfg(all(feature = "sha2", not(any(target_os = "solana", target_os = "zkvm"))))]
#[derive(Clone, Default)]
pub struct Hasher {
    hasher: Sha256,
}

#[cfg(all(feature = "sha2", not(any(target_os = "solana", target_os = "zkvm"))))]
impl Hasher {
    pub fn hash(&mut self, val: &[u8]) {
        self.hasher.update(val);
    }
    pub fn hashv(&mut self, vals: &[&[u8]]) {
        for val in vals {
            self.hash(val);
        }
    }
    pub fn result(self) -> Hash {
        let bytes: [u8; solana_hash::HASH_BYTES] = self.hasher.finalize().into();
        bytes.into()
    }
}

#[cfg(any(target_os = "solana", target_os = "zkvm"))]
pub use solana_define_syscall::definitions::sol_sha256;

/// Return a Sha256 hash for the given data.
pub fn hashv(vals: &[&[u8]]) -> Hash {
    // Perform the calculation inline, calling this from within a program is
    // not supported
    #[cfg(not(any(target_os = "solana", target_os = "zkvm")))]
    {
        #[cfg(feature = "sha2")]
        {
            let mut hasher = Hasher::default();
            hasher.hashv(vals);
            hasher.result()
        }
        #[cfg(not(feature = "sha2"))]
        {
            core::hint::black_box(vals);
            panic!("hashv is only available on target `solana` or with the `sha2` feature enabled on this crate")
        }
    }
    // Call via a system call to perform the calculation
    #[cfg(any(target_os = "solana", target_os = "zkvm"))]
    {
        let mut hash_result = [0; solana_hash::HASH_BYTES];
        unsafe {
            sol_sha256(
                vals as *const _ as *const u8,
                vals.len() as u64,
                &mut hash_result as *mut _ as *mut u8,
            );
        }
        Hash::new_from_array(hash_result)
    }
}

/// Return a Sha256 hash for the given data.
pub fn hash(val: &[u8]) -> Hash {
    hashv(&[val])
}
