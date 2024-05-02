use esp_idf_svc::systime::EspSystemTime;
use ultrasonic::startup::App;
use ultrasonic::ultrasonic as sensor;
use esp_idf_hal::{
    delay::FreeRtos,
    gpio:: PinDriver,
    peripherals::Peripherals,
};
use crossbeam_channel::bounded;

static ULTRASONIC_STACK_SIZE: usize = 2000;

fn main() -> anyhow::Result<()>{
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    //creating the http server
    let app = App::spawn()?;

    //ultrasonic sensor
    let ultrasonic = sensor::Ultrasonic::new()?;

    //reset button
    let peripherals = Peripherals::take().unwrap();
    let mut button = PinDriver::input(peripherals.pins.gpio15)?;

    let (tx, rx) = bounded::<f32>(1);


    let _ultrasonic_thread = std::thread::Builder::new()
        .stack_size(ULTRASONIC_STACK_SIZE)
        .spawn(move || ultrasonic_thread_function(ultrasonic, tx))?;

    let mut distance = 0.0;

    loop {
        match rx.try_recv() {
            Ok(x) => println!("{}", x),
            Err(_) => {}
        }
    }
}

fn ultrasonic_thread_function(
    mut ultrasonic: sensor::Ultrasonic,
    tx: crossbeam_channel::Sender<f32>,
) -> anyhow::Result<()> {
    let mut distance_status = 0.0;

    loop {
        //clean input
        ultrasonic.trigger.set_low()?;
        FreeRtos::delay_ms(2);
        // Send a 10ms pulse to the trigger pin to start the measurement
        ultrasonic.trigger.set_high()?;
        FreeRtos::delay_ms(10);
        ultrasonic.trigger.set_low()?;

        while !ultrasonic.echo.is_high() {}

        let start_time = EspSystemTime {}.now().as_micros();
        while ultrasonic.echo.is_high() {}
        let end_time = EspSystemTime {}.now().as_micros();

        let pulse_duration = end_time - start_time;
        distance_status = (pulse_duration as f32 * 0.0343) / 2.0;
        println!("distance: {}", distance_status);
        tx.send(distance_status)?;
        FreeRtos::delay_ms(2000);
    }
}
