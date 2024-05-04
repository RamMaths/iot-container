use esp_idf_svc::systime::EspSystemTime;
use crate::ultrasonic::Ultrasonic;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::task::watchdog::TWDTDriver;


pub fn ultrasonic_thread_function(
    twdt: &mut TWDTDriver,
    mut ultrasonic: Ultrasonic,
    tx: crossbeam_channel::Sender<f32>,
) -> anyhow::Result<()> {
    let mut _distance_status = 0.0;
    let mut start_measurment_time = EspSystemTime {}.now().as_micros();
    let interval = 1000000;
    let mut first = true;

    let mut watchdog = twdt.watch_current_task()?;
        
    loop {
        let actual_time = EspSystemTime {}.now().as_micros();
        //clean input

        if actual_time - start_measurment_time > interval || first {
            ultrasonic.trigger.set_low()?;
            FreeRtos::delay_ms(2);
            // Send a 10ms pulse to the trigger pin to start the measurement
            ultrasonic.trigger.set_high()?;
            FreeRtos::delay_ms(10);
            ultrasonic.trigger.set_low()?;

            while !ultrasonic.echo.is_high() { watchdog.feed()?; }

            let start_time = EspSystemTime {}.now().as_micros();
            while ultrasonic.echo.is_high() { watchdog.feed()?; }
            let end_time = EspSystemTime {}.now().as_micros();

            let pulse_duration = end_time - start_time;
            _distance_status = (pulse_duration as f32 * 0.0343) / 2.0;
            tx.send(_distance_status)?;
            start_measurment_time = EspSystemTime {}.now().as_micros();
            first = false;
        }
    }
}
