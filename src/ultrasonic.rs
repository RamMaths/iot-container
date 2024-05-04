use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_hal::gpio::{
    AnyOutputPin,
    Output,
    OutputPin,
    Input,
    IOPin,
    AnyIOPin,
    PinDriver
};

pub struct Ultrasonic {
    pub trigger: PinDriver<'static, AnyOutputPin, Output>,
    pub echo: PinDriver<'static, AnyIOPin, Input>
}

impl Ultrasonic {
    pub fn new() -> anyhow::Result<Ultrasonic> {
        let peripherals = unsafe { Peripherals::new() };
        let trigger = PinDriver::output(peripherals.pins.gpio4.downgrade_output())?;
        let echo = PinDriver::input(peripherals.pins.gpio5.downgrade())?;

        Ok(
            Ultrasonic {
                trigger,
                echo
            }
        )
    }
}
