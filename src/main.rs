mod dom;
mod renderer;
mod view;
use dom::dom::Dom;
use dom::element::Element;

use skia_safe::{AlphaType, Canvas, Color, Color4f, ColorType, Font, FontMgr, FontStyle, ImageInfo, Paint, PaintStyle, Point, Rect, Surface, Typeface, surfaces};
use std::num::NonZeroU32;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};

use crate::dom::element;
use crate::dom::layoutengine::LayoutEngine;
use crate::renderer::skiarenderer::SkiaRenderer;

#[path = "utils/winit_app.rs"]
mod winit_app;

#[cfg(not(target_os = "android"))]
fn main() {
    entry(EventLoop::new().unwrap())
}
//test
pub(crate) fn entry(event_loop: EventLoop<()>) {
    let app = winit_app::WinitAppBuilder::with_init(
        |elwt| {
            let window = winit_app::make_window(elwt, |w| w);

            let context = softbuffer::Context::new(window.clone()).unwrap();

            (window, context)
        },
        |_elwt, (window, context)| softbuffer::Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(|(window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::Resized(size) } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("Resized fired before Resumed or after Suspended");
                    return;
                };

                if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    surface.resize(width, height).unwrap();
                }
            }
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = window.inner_size();
                if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    // buffer.present().unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();

                    // Calculate buffer dimensions
                    let width = width.get() as usize;
                    let height = height.get() as usize;
                    let stride = width * 4; // 4 bytes per pixel (BGRA8888)

                    // Create Skia surface directly from the buffer
                    let info = ImageInfo::new((width as i32, height as i32), ColorType::BGRA8888, AlphaType::Unpremul, None);

                    // Safe because we know the buffer dimensions and format match our requirements
                    let bytes = unsafe {
                        std::slice::from_raw_parts_mut(
                            buffer.as_mut_ptr() as *mut u8,
                            height * stride, // Total bytes = height * row stride
                        )
                    };

                    if let Some(mut skia_surface) = surfaces::wrap_pixels(&info, bytes, stride, None) {
                        let canvas = skia_surface.canvas();
                        canvas.clear(Color::BLACK);

                        let mut dom = Dom::new();
                        let root = Element::new(element::Color::new(1.0, 1.0, 0.0, 1.0));
                        let root_node_id = dom.create_element(root);
                        dom.set_root(root_node_id);

                        view::create_view(&mut dom, root_node_id);

                        LayoutEngine::compute_layout(&mut dom, width as f32, height as f32);
                        SkiaRenderer::draw_dom(&canvas, &dom);
                    }

                    buffer.present().unwrap();
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
                    ..
                },
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    });

    winit_app::run_app(event_loop, app);
}
