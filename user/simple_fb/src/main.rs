#![no_std]
#![no_main]
#![feature(const_generics, alloc_error_handler)]

extern crate alloc;
extern crate rlibc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{mem::MaybeUninit, panic::PanicInfo};
use gfxconsole::{Bgr32, Format, Framebuffer, Pixel};
use libpebble::{
    caps::{CapabilitiesRepr, CAP_EARLY_LOGGING, CAP_GET_FRAMEBUFFER, CAP_PADDING, CAP_SERVICE_USER},
    early_logger::EarlyLogger,
    syscall::{self, FramebufferInfo, PixelFormat},
};
use linked_list_allocator::LockedHeap;
use log::info;
use ptah::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestMessage {
    id: usize,
    message: String,
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    syscall::early_log("Hello from FB").unwrap();
    // Initialise the heap
    const HEAP_START: usize = 0x600000000;
    const HEAP_SIZE: usize = 0x4000;
    let heap_memory_object = syscall::create_memory_object(HEAP_START, HEAP_SIZE, true, false).unwrap();
    unsafe {
        syscall::map_memory_object(heap_memory_object, libpebble::ZERO_HANDLE, 0x0 as *mut usize).unwrap();
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    log::set_logger(&EarlyLogger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    info!("Simple framebuffer driver is running!");

    /*
     * Test out the service stuff. We want the echo service.
     */
    let echo_channel = syscall::subscribe_to_service("echo.echo").expect("Failed to subscribe to echo service :(");
    let message = {
        let mut message_bytes = Vec::new();
        ptah::to_wire(&TestMessage { id: 43, message: "Hello, World!".to_string() }, &mut message_bytes).unwrap();
        message_bytes
    };
    syscall::send_message(echo_channel, &message, &[]).unwrap();
    let mut messages_received = 0;
    loop {
        let mut bytes = [0u8; 256];
        match syscall::get_message(echo_channel, &mut bytes, &mut []) {
            Ok((bytes, _handles)) => {
                info!("Echo sent message back: {:x?}", bytes);
                let received_message: TestMessage = ptah::from_wire(bytes).unwrap();
                info!("Message from echo: {:?}", received_message);
                messages_received += 1;

                // NOTE: this can be changed to receive a total number of messages overall
                if messages_received == 1 {
                    break;
                }
            }
            Err(syscall::GetMessageError::NoMessage) => syscall::yield_to_kernel(),
            Err(err) => panic!("Error getting message from echo: {:?}", err),
        }
    }

    let framebuffer = make_framebuffer();
    framebuffer.clear(Bgr32::pixel(0xaa, 0xaa, 0xaa, 0xff));
    framebuffer.draw_rect(100, 100, 300, 450, Bgr32::pixel(0xcc, 0x00, 0x00, 0xff));
    framebuffer.draw_string(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        200,
        400,
        Bgr32::pixel(0x00, 0x00, 0x00, 0xff),
    );

    loop {
        info!("Yielding from FB");
        syscall::yield_to_kernel();
    }
}

fn make_framebuffer() -> Framebuffer<Bgr32> {
    let (framebuffer_handle, framebuffer_info) = {
        let mut framebuffer_info: MaybeUninit<FramebufferInfo> = MaybeUninit::uninit();

        let framebuffer_handle =
            syscall::get_framebuffer(framebuffer_info.as_mut_ptr()).expect("Failed to get handle to framebuffer!");

        (framebuffer_handle, unsafe { framebuffer_info.assume_init() })
    };

    let mut framebuffer_address: MaybeUninit<usize> = MaybeUninit::uninit();
    unsafe {
        syscall::map_memory_object(framebuffer_handle, libpebble::ZERO_HANDLE, framebuffer_address.as_mut_ptr())
            .unwrap();
    }
    let framebuffer_address = unsafe { framebuffer_address.assume_init() };

    assert_eq!(framebuffer_info.pixel_format, PixelFormat::BGR32);

    Framebuffer {
        ptr: framebuffer_address as *mut Pixel<Bgr32>,
        width: framebuffer_info.width as usize,
        height: framebuffer_info.height as usize,
        stride: framebuffer_info.stride as usize,
    }
}

#[panic_handler]
pub fn handle_panic(info: &PanicInfo) -> ! {
    log::error!("PANIC: {}", info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Alloc error: {:?}", layout);
}

#[used]
#[link_section = ".caps"]
pub static mut CAPS: CapabilitiesRepr<4> =
    CapabilitiesRepr::new([CAP_EARLY_LOGGING, CAP_GET_FRAMEBUFFER, CAP_SERVICE_USER, CAP_PADDING]);
