use esp_idf_svc::systime::EspSystemTime;
use ultrasonic::startup::App;
use ultrasonic::ultrasonic as sensor;
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull},
    peripherals::Peripherals,
};
use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;

static ULTRASONIC_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;

fn main() -> anyhow::Result<()>{
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    //creating the http server
    let app = App::spawn()?;

    //ultrasonic sensor
    let mut ultrasonic = sensor::set_ultrasonic_sensor()?;

    // //distance
    let mut distance = 0.0;

    //reset button
    let peripherals = Peripherals::take().unwrap();
    let mut button = PinDriver::input(peripherals.pins.gpio15)?;

    let (tx, rx) = bounded::<f32>(1);

    let _ultrasonic_thread = std::thread::Builder::new()
        .stack_size(ULTRASONIC_STACK_SIZE)
        .spawn(move || blinky_thread_function(led_pin, rx))
        .unwrap();

     let mut flag = false;
     let mut counter = 100;

     loop {
    
         // if flag {
         //     app.client.post_request(0)?;
         //     flag = false;
         //     counter = 100;
         // } else {
         //     FreeRtos::delay_ms(100);
         //     counter -= 1;
         //     if counter < 0 {
         //         flag = true;
         //     }
         // }
    
     }
    Ok(())
}

fn ultrasonic_thread_function(
    ultrasonic: ultrasonic::Ultrasonic,
    tx: crossbeam_channel::Sender<f32>,
) {
    let mut distance_status = 0.0;

    loop {
        //clean input
        ultrasonic.trigger.set_low().unwrap();
        FreeRtos::delay_ms(2);
        // Send a 10ms pulse to the trigger pin to start the measurement
        ultrasonic.trigger.set_high().unwrap();
        FreeRtos::delay_ms(10);
        ultrasonic.trigger.set_low().unwrap();

        while !ultrasonic.echo.is_high() {}

        let start_time = EspSystemTime {}.now().as_micros();
        while ultrasonic.echo.is_high() {}
        let end_time = EspSystemTime {}.now().as_micros();

        let pulse_duration = end_time - start_time;
        distance_status = (pulse_duration as f32 * 0.0343) / 2.0;
        println!("distance: {}", distance_status);
        tx.send(distance_status).unwrap();
        FreeRtos::delay_ms(2000);
    }
}

fn blinky_thread_function(
    rx: crossbeam_channel::Receiver<f32>
) {
    let mut blinky_status = false;
    loop {
        match rx.try_recv() {
            Ok(x) => blinky_status = x,
            Err(_) => {}
        }

        if blinky_status {
            led_pin.set_low().unwrap();
            println!("LED ON");
            FreeRtos::delay_ms(1000);

            led_pin.set_high().unwrap();
            println!("LED OFF");
        }
        FreeRtos::delay_ms(1000);
    }
}
