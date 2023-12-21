// This trait is made to be generic to allow platform-specific implementations
// of the serial interface.
pub trait PlatformSerial<Word>:
    embedded_hal::serial::Read<Word> + embedded_hal::serial::Write<Word> + core::default::Default
{
}
