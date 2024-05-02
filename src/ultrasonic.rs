use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_hal::gpio::{
    AnyOutputPin,
    AnyInputPin,
    Output,
    OutputPin,
    Input,
    IOPin,
    AnyIOPin,
    PinDriver,
    Pull
};

pub struct Ultrasonic {
    pub trigger: PinDriver<'static, AnyOutputPin, Output>,
    pub echo: PinDriver<'static, AnyIOPin, Input>
}

pub fn set_ultrasonic_sensor() -> anyhow::Result<Ultrasonic> {
    let peripherals = unsafe { Peripherals::new() };
    let trigger = PinDriver::output(peripherals.pins.gpio8.downgrade_output())?;
    let echo = PinDriver::input(peripherals.pins.gpio5.downgrade())?;

    Ok(
        Ultrasonic {
            trigger,
            echo
        }
    )
}
