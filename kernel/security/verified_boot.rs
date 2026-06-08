pub struct VerifiedBoot;

impl VerifiedBoot {
    pub fn verify_kernel_signature(
        image_hash: &[u8],
        expected_hash: &[u8],
    ) -> bool {
        image_hash == expected_hash
    }

    pub fn verify_boot_chain() -> bool {
        // Placeholder:
        // UEFI -> Bootloader -> Kernel -> Init

        true
    }
}
