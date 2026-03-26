#![no_std]

use core::{error::Error, ffi::c_void};

use alloc::boxed::Box;
use foam_common::FoamRenderer;
use psp::{
    BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    sys::{self, GuState, sceGuColor, sceGuDrawArray, sceGuGetMemory},
    vram_alloc::{SimpleVramAllocator, VramMemChunk, get_vram_allocator},
};

#[repr(C)]
struct Vertex {
    pub u: u16,
    pub v: u16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

extern crate alloc;

pub struct Renderer {
    allocator: SimpleVramAllocator,
    /// Framebuffer pointer 0, used as a draw buffer
    fbp0: *mut u8,
    /// Framebuffer pointer 1, used as a display buffer
    fbp1: *mut u8,
    /// Z buffer pointer, used as a depth buffer
    zbp: *mut u8,
}

impl Renderer {
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
    pub fn init(&mut self) {
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

impl FoamRenderer for Renderer {
    fn clear(&mut self, color: u32) {
        let new_color = (0xFF << 24) | color;
        unsafe {
            sys::sceGuStart(
                sys::GuContextType::Direct,
                &raw mut LIST as *mut _ as *mut c_void,
            );
            sys::sceGuClearColor(new_color);
            sys::sceGuClearDepth(0);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::DEPTH_BUFFER_BIT,
            );
        }
    }
    fn end_drawing(&mut self) {
        unsafe {
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
            let new_color = (0xFF << 24) | color;
            sceGuColor(new_color);
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
}
