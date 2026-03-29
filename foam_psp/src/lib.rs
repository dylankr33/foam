#![no_std]
use alloc::{
    boxed::Box,
    rc::{Rc, Weak},
    vec::Vec,
};
use core::{cell::RefCell, error::Error, ffi::c_void};
use foam_common::{Button, Event, EventHandler, FoamBackend, FoamCanvas, rgb_to_abgr};
use psp::{
    BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    sys::{self, CtrlButtons, GuState, sceGuColor, sceGuDrawArray, sceGuGetMemory},
    vram_alloc::{SimpleVramAllocator, get_vram_allocator},
};
#[repr(C, align(4))]
struct Vertex {
    pub u: u16,
    pub v: u16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

extern crate alloc;

#[derive(Clone)]
pub struct PspCanvas {
    renderer: *mut PspRenderer,
}

impl FoamCanvas for PspCanvas {
    fn draw_square(&mut self, color: u32, w: u16, h: u16, x: i16, y: i16) {
        unsafe { self.renderer.as_mut_unchecked() }.draw_square(color, w, h, x, y);
    }
}

pub struct PspRenderer {
    allocator: SimpleVramAllocator,
    /// Framebuffer pointer 0, used as a draw buffer
    fbp0: *mut u8,
    /// Framebuffer pointer 1, used as a display buffer
    fbp1: *mut u8,
    /// Z buffer pointer, used as a depth buffer
    zbp: *mut u8,
}

impl PspRenderer {
    /// Create a new app
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let allocator = get_vram_allocator().unwrap();
        let fbp0 = allocator
            .alloc_texture_pixels(
                BUF_WIDTH,
                SCREEN_HEIGHT,
                psp::sys::TexturePixelFormat::Psm8888,
            )
            .as_mut_ptr_from_zero();
        let fbp1 = allocator
            .alloc_texture_pixels(
                BUF_WIDTH,
                SCREEN_HEIGHT,
                psp::sys::TexturePixelFormat::Psm8888,
            )
            .as_mut_ptr_from_zero();
        let zbp = allocator
            .alloc_texture_pixels(
                BUF_WIDTH,
                SCREEN_HEIGHT,
                psp::sys::TexturePixelFormat::Psm4444,
            )
            .as_mut_ptr_from_zero();
        Ok(Self {
            allocator,
            fbp0,
            fbp1,
            zbp,
        })
    }

    /// Initialize the app
    pub fn init(&self) {
        unsafe {
            sys::sceGuInit();
            sys::sceGuStart(
                sys::GuContextType::Direct,
                &raw mut LIST as *mut _ as *mut c_void,
            );
            sys::sceGuDrawBuffer(
                sys::DisplayPixelFormat::Psm8888,
                self.fbp0 as _,
                BUF_WIDTH as i32,
            );
            sys::sceGuDispBuffer(
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                self.fbp1 as _,
                BUF_WIDTH as i32,
            );
            sys::sceGuDepthBuffer(self.zbp as _, BUF_WIDTH as i32);
            sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
            sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuDepthRange(65535, 0);
            sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuEnable(GuState::ScissorTest);
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblank();
            sys::sceGuDisplay(true);
        }
    }
}

pub struct PspBackend {
    renderer: *mut PspRenderer,
    canvas: PspCanvas,
}

impl PspBackend {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let renderer = &mut PspRenderer::new()?;
        renderer.init();
        let canvas = PspCanvas { renderer };
        Ok(Self { canvas, renderer })
    }
}

impl FoamBackend for PspBackend {
    fn poll_event(&mut self) -> Vec<Event> {
        unsafe { self.renderer.as_mut_unchecked() }.poll_event()
    }

    fn draw(&mut self, cb: &dyn Fn(&mut dyn FoamCanvas)) {
        unsafe { self.renderer.as_mut_unchecked() }.draw(cb, &mut self.canvas);
    }
}

impl PspRenderer {
    pub fn draw(&mut self, cb: &dyn Fn(&mut dyn FoamCanvas), canvas: &mut dyn FoamCanvas) {
        unsafe {
            sys::sceGuStart(
                sys::GuContextType::Direct,
                &raw mut LIST as *mut _ as *mut c_void,
            );
            sys::sceGuClearColor(0xffffffff);
            sys::sceGuClearDepth(0);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::DEPTH_BUFFER_BIT,
            );
            cb(canvas);

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }
    fn draw_square(&mut self, color: u32, w: u16, h: u16, x: i16, y: i16) {
        unsafe {
            let vertices = sceGuGetMemory(2 * size_of::<Vertex>() as i32) as *mut Vertex;
            ((*vertices.wrapping_add(0)).x) = x;
            ((*vertices.wrapping_add(0)).y) = y;
            ((*vertices.wrapping_add(1)).x) = x + w as i16;
            ((*vertices.wrapping_add(1)).y) = y + h as i16;
            let color = rgb_to_abgr(color);
            sceGuColor(color);
            sceGuDrawArray(
                sys::GuPrimitive::Sprites,
                sys::VertexType::TEXTURE_16BIT
                    | sys::VertexType::VERTEX_16BIT
                    | sys::VertexType::TRANSFORM_2D,
                2,
                0 as *const _,
                vertices as *const c_void,
            );
        }
    }
    pub fn poll_event(&mut self) -> alloc::vec::Vec<Event> {
        let mut events: Vec<Event> = Vec::new();
        unsafe {
            let mut pad_data = sys::SceCtrlData::default();
            sys::sceCtrlReadBufferPositive(&mut pad_data, 1);
            if !pad_data.buttons.is_empty() {
                use Button::*;
                use Event::*;
                if pad_data.buttons.contains(CtrlButtons::UP) {
                    events.push(Pad(Up));
                }
                if pad_data.buttons.contains(CtrlButtons::DOWN) {
                    events.push(Pad(Down));
                }
                if pad_data.buttons.contains(CtrlButtons::RIGHT) {
                    events.push(Pad(Right));
                }
                if pad_data.buttons.contains(CtrlButtons::LEFT) {
                    events.push(Pad(Left));
                }
            }
        }

        events
    }
}
