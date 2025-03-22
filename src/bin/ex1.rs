#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::{info, panic};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::config::Config;
use embassy_rp::pwm::SetDutyCycle;
use embassy_rp::{
    gpio, init, peripherals,
    pwm::{Config as PwmConfig, Pwm},
};
use embassy_time::{Duration, Timer};
use gpio::{Input, Pull};
use gpio::{Level, Output};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Config::default());

    let mut button1 = Input::new(peripherals.PIN_2, Pull::Up);
    let mut button2 = Input::new(peripherals.PIN_3, Pull::Up);
    let mut button3 = Input::new(peripherals.PIN_4, Pull::Up);
    let mut button4 = Input::new(peripherals.PIN_6, Pull::Up);

    loop{
        if button1.is_low() | button2.is_low() | button3.is_low() | button4.is_low(){
            info!("Team name: Pico 3W");
        }
        Timer::after(Duration::from_millis(200)).await;
    }
}