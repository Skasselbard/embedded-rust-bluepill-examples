#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use cortex_m_rt::entry;

use core::{mem::MaybeUninit, panic::PanicInfo};

use embedded_rust_h2al::{events, Component, ComponentsBuilder};
use embedded_rust_hardware_init::device_config;

#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // log::error!("panic: {}", info);
    cortex_m_semihosting::hprintln!("panic: {}", info).unwrap();
    cortex_m::interrupt::disable();
    loop {}
}
#[device_config(
    Stm32f1xx:
      sys:
        heap_size: [10, "kb"]
        sys_clock: [36, "mhz"]
      gpios:
        - ["PA0", "input", "pull_up", "falling"]
        - ["PC13", "output", "push_pull"]
      // pwm:
      //   - timer: "Tim2"
      //     pins: ["PA1"]
      //     frequency: [10, "khz"]
      // serial:
      //   - usart1:
      //       tx: "PB6"
      //       rx: "PB7"
      //       baud: 9600
)]
struct BluePill;

#[entry]
fn main() -> ! {
    static mut ca: [MaybeUninit<Component>; 2] = ComponentsBuilder::allocate_array();
    let components = BluePill::init();
    let mut cb = ComponentsBuilder::new(ca);
    let c = unsafe { cb.finalize() };
    loop {}
}

pub async fn test_task() {
    loop {}
}

// macro_rules! to_target_endianess {
//     ($int:expr) => {
//         if cfg!(target_endian = "big") {
//             $int.to_be_bytes()
//         } else {
//             $int.to_le_bytes()
//         }
//     };
// }

// enum Level {
//     Full,
//     High,
//     Half,
//     Low,
//     Off,
// }

// struct Brightness {
//     level: Level,
// }

// impl Brightness {
//     fn next(&mut self) -> f32 {
//         match self.level {
//             Level::Full => {
//                 self.level = Level::High;
//                 0.75f32
//             }
//             Level::High => {
//                 self.level = Level::Half;
//                 0.5f32
//             }
//             Level::Half => {
//                 self.level = Level::Low;
//                 0.25f32
//             }
//             Level::Low => {
//                 self.level = Level::Off;
//                 0.0f32
//             }
//             Level::Off => {
//                 self.level = Level::Full;
//                 1.0f32
//             }
//         }
//     }
// }

// pub async fn test_task() {
//     let mut button_events = BluePill::get_resource("event:gpio/pa0").unwrap();
//     let mut led = BluePill::get_resource("digital:gpio/pc13").unwrap();
//     let mut brightness = Brightness { level: Level::Off };
//     let mut pwm = BluePill::get_resource("percent:pwm/pa1").unwrap();
//     let mut usart1 = BluePill::get_resource("bus:serial/usart1").unwrap();
//     pwm.write(&to_target_endianess!(brightness.next()))
//         .await
//         .unwrap();
//     let mut led_state = false;
//     let mut buf = [0; 6];
//     loop {
//         usart1.write_all("ABCDEF".as_bytes()).await.unwrap();
//         log::info!("written");
//         usart1.read(&mut buf).await.unwrap();
//         log::info!("{:?}", buf);
//     }
// }
