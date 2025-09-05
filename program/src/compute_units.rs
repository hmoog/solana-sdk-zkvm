/// Return the remaining compute units the program may consume
#[inline]
pub fn sol_remaining_compute_units() -> u64 {
    #[cfg(any(target_os = "solana", target_os = "zkvm"))]
    unsafe {
        crate::syscalls::sol_remaining_compute_units()
    }

    #[cfg(not(any(target_os = "solana", target_os = "zkvm")))]
    {
        crate::program_stubs::sol_remaining_compute_units()
    }
}
