#![no_std]
use alloc::{
    boxed::Box,
    rc::{Rc, Weak},
    vec::Vec,
};
use core::{cell::RefCell, error::Error, ffi::c_void, prelude, ptr};
use foam_common::{FoamBackend, FoamCanvas, prelude::*, rgb_to_abgr, shapes};
use psp::{
    Align16, BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    sys::{
        self, CtrlButtons, GuState, UtilityHtmlViewerOption, VertexType, sceGuColor,
        sceGuDrawArray, sceGuGetMemory, sceGumDrawArray, sceGumLoadMatrix, sceGumTranslate,
    },
    vram_alloc::{SimpleVramAllocator, get_vram_allocator},
};

const fn vertex_to_psp_vertex<const N: usize>(data: [Vertex; N]) -> [PspVertex; N] {
    let mut bleh = [const {
        PspVertex {
            u: 0.0,
            v: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }; N];
    let mut i = 0;
    while i < N {
        bleh[i as usize] = PspVertex {
            u: data[i as usize].u,
            v: data[i as usize].v,
            x: data[i as usize].x,
            y: data[i as usize].y,
            z: data[i as usize].z,
        };
        i += 1;
    }
    bleh
}

static PSP_CUBE: Align16<[PspVertex; shapes::cube::VERTICES.len()]> =
    Align16(vertex_to_psp_vertex(shapes::cube::VERTICES));

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

#[repr(C, packed)]
struct PspVertex {
    u: f32,
    v: f32,
    x: f32,
    y: f32,
    z: f32,
}

extern crate alloc;

#[derive(Clone)]
pub struct PspCanvas {
    renderer: Rc<RefCell<PspRenderer>>,
}

impl FoamCanvas for PspCanvas {
    fn draw_cube(&self, color: u32, position: (f32, f32, f32)) {
        self.renderer.borrow_mut().draw_mesh(
            color,
            Align16(PSP_CUBE.0.as_ptr()),
            36,
            ptr::null_mut(),
            position,
        );
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
            sys::sceGuDepthFunc(sys::DepthFunc::Always);
            sys::sceGuEnable(GuState::DepthTest);
            sys::sceGuFrontFace(sys::FrontFaceDirection::CounterClockwise);
            sys::sceGuShadeModel(sys::ShadingModel::Smooth);
            sys::sceGuDisable(GuState::CullFace);
            sys::sceGuEnable(GuState::ClipPlanes);
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblank();
            sys::sceGuDisplay(true);
        }
    }
}

pub struct PspBackend {
    renderer: Rc<RefCell<PspRenderer>>,
    canvas: PspCanvas,
}

impl PspBackend {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let renderer = Rc::new(RefCell::new(PspRenderer::new()?));
        renderer.borrow_mut().init();
        let canvas = PspCanvas {
            renderer: renderer.clone(),
        };
        Ok(Self { canvas, renderer })
    }
}

impl FoamBackend for PspBackend {
    fn poll_event(&mut self) -> Vec<Event> {
        self.renderer.borrow_mut().poll_event()
    }

    fn draw(&mut self, cb: &dyn Fn(&mut dyn FoamCanvas)) {
        self.renderer.borrow_mut().start_frame();
        cb(&mut self.canvas);
        self.renderer.borrow_mut().end_frame();
    }
}

impl PspRenderer {
    pub fn start_frame(&mut self) {
        unsafe {
            sys::sceGuStart(
                sys::GuContextType::Direct,
                &raw mut LIST as *mut _ as *mut c_void,
            );
            sys::sceGuClearColor(0xffaaaaaa);
            sys::sceGuClearDepth(0xffff);
            sys::sceGuClear(
                sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::DEPTH_BUFFER_BIT,
            );
            sys::sceGumMatrixMode(sys::MatrixMode::Projection);
            sys::sceGumLoadIdentity();
            sys::sceGumPerspective(75.0, 16.0 / 9.0, 0.5, 1000.0);
            sys::sceGumMatrixMode(sys::MatrixMode::View);
            sys::sceGumLoadIdentity();
        }
    }
    pub fn end_frame(&mut self) {
        unsafe {
            sys::sceGuFinish();
            sys::sceKernelDcacheWritebackAll();

            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }

    pub fn draw_mesh(
        &mut self,
        color: u32,
        data: Align16<*const PspVertex>,
        count: i32,
        indices: *mut u16,
        pos: (f32, f32, f32),
    ) {
        unsafe {
            sys::sceGumMatrixMode(sys::MatrixMode::Model);
            sys::sceGumLoadIdentity();
            sys::sceGumTranslate(&sys::ScePspFVector3 {
                x: pos.0,
                y: pos.1,
                z: pos.2,
            });
            let color = rgb_to_abgr(color);
            sys::sceGuColor(color);
            sys::sceGumDrawArray(
                sys::GuPrimitive::Triangles,
                VertexType::TEXTURE_32BITF | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D,
                count,
                indices as *mut c_void,
                data.0 as *const _,
            );
        }
    }

    fn draw_square(&mut self, color: u32, w: f32, h: f32, x: f32, y: f32) {
        unsafe {
            let vertices = sceGuGetMemory(2 * size_of::<Vertex>() as i32) as *mut Vertex;
            (*vertices.wrapping_add(0)).x = x;
            (*vertices.wrapping_add(0)).y = y;
            (*vertices.wrapping_add(1)).x = x;
            (*vertices.wrapping_add(1)).y = y;
            let color = rgb_to_abgr(color);
            sceGuColor(color);
            sceGumDrawArray(
                sys::GuPrimitive::Sprites,
                sys::VertexType::TEXTURE_32BITF
                    | sys::VertexType::VERTEX_32BITF
                    | sys::VertexType::TRANSFORM_3D,
                2,
                0 as *const _,
                vertices as *mut c_void,
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
