const GPIO_LED: u8 = 2;

fn main() {
    let mut pin = rppal::gpio::Gpio::new()
        .unwrap()
        .get(GPIO_LED)
        .unwrap()
        .into_output();

    loop {
        pin.toggle();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
