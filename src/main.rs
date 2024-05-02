use esp_idf_svc::systime::EspSystemTime;
use esp_idf_hal::delay::FreeRtos;
use ultrasonic::startup::App;
use ultrasonic::ultrasonic as sensor;


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

    // let mut flag = false;
    // let mut counter = 100;

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

        distance = (pulse_duration as f32 * 0.0343) / 2.0;

        println!("distance: {}", distance);
        FreeRtos::delay_ms(5000);
    }
}


