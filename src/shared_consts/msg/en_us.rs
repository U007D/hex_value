pub const ERR_ADDRESS_TOO_LARGE: &str = "Error: Address too large.  Please limit values to \
                                         `u128::MAX` (0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff) \
                                         or less";
pub const ERR_FILE_CREATE: &str = "An error occurred attempting to create a file";
pub const ERR_FILE_OPEN: &str = "An error occurred attempting to open a file";
pub const ERR_FILE_READ: &str = "An error occurred performing file read I/O";
pub const ERR_FILE_WRITE: &str = "An error occurred performing file write I/O";
pub const ERR_IO: &str = "Error performing I/O";
pub const ERR_NON_ASCII_INPUT: &str = "Error: Received input containing non-ASCII characters.";
pub const ERR_NON_HEXADECIMAL_INPUT: &str = "Error: Received input containing non-hexadecimal \
                                             characters.  Please limit input to ASCII 0-9, a-f or \
                                             A-F.  Additionally, input may be prefixed with '0x' \
                                             and may contain '_' separators.";
