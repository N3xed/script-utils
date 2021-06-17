use script_utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_file("https://github.com/espressif/esp-idf/raw/aaf12390eb14b95589acd98db5c268a2e56bb67e/components/spi_flash/spi_flash_rom_patch.c", "test.c")?;
    Ok(())
}