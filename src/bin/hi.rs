#![no_std]
#![no_main]

use core::cmp::min;

use cortex_m_rt::entry;
use defmt::{info, panic};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::adc::InterruptHandler;
use embassy_rp::config::Config;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::pwm::SetDutyCycle;
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut button1 = Input::new(p.PIN_2, Pull::Up);
    let mut button2 = Input::new(p.PIN_3, Pull::Up);
    let mut button3 = Input::new(p.PIN_4, Pull::Up);
    let mut button4 = Input::new(p.PIN_6, Pull::Up);
    loop {
        button1.wait_for_falling_edge().await;
        info!("Team name: Pico 3W");
        button2.wait_for_falling_edge().await;
        info!("Team name: Pico 3W");
        button3.wait_for_falling_edge().await;
        info!("Team name: Pico 3W");
        button4.wait_for_falling_edge().await;
        info!("Team name: Pico 3W");
    }
}
