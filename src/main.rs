mod dom;
mod renderer;
mod view;
use debugtools::DebugTools;
use dom::dom::Dom;
use dom::element::Element;


use skia_safe::{AlphaType, Canvas, Color, Color4f, ColorType, Font, FontMgr, FontStyle, ImageInfo, Paint, PaintStyle, Point, Rect, Surface, Typeface, surfaces};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use winit::window::Window;

use crate::dom::layoutengine::LayoutEngine;
use crate::dom::{debugtools, element, styleengine};
use crate::renderer::skiarenderer::SkiaRenderer;

#[path = "utils/winit_app.rs"]
mod winit_app;

#[cfg(not(target_os = "android"))]
fn main() {
    entry(EventLoop::new().unwrap())
}
//test
pub(crate) fn entry(event_loop: EventLoop<()>) {
    let mut debug_tools = DebugTools::new();
    debug_tools.log("test");
    let mut dom = Dom::new();
    let root = Element::new();
    let root_node_id = dom.create_element(root);
    dom.set_root(root_node_id);
    view::create_view(&mut dom, root_node_id);

    let app = winit_app::WinitAppBuilder::with_init(
        |elwt| {
            let window = winit_app::make_window(elwt, |w| w);

            let context = softbuffer::Context::new(window.clone()).unwrap();

            (window, context)
        },
        |_elwt, (window, context)| softbuffer::Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(move |(window, _context), surface, event, elwt| {
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
            Event::WindowEvent {
                window_id,
                event: WindowEvent::KeyboardInput { device_id, event, is_synthetic },
            } => {
                //debug_tools.log(event.physical_key::);
                if let (PhysicalKey::Code(KeyCode::F12), ElementState::Pressed) = (event.physical_key, event.state) {
                    println!("F12 pressed!");
                    debug_tools.log("F12");
                    window.request_redraw();
                    // Your logic here
                }
            }

            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                //buffer_from_surface(window, surface);
                let size = window.inner_size();
                if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    // buffer.present().unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();

                    // Calculate buffer dimensions
                    debug_tools.log("render");
                    SkiaRenderer::render(
                        &mut dom,
                        buffer.as_mut(),
                        width.get() as usize,
                        height.get() as usize,
                        Some(&mut debug_tools), // or None if debug tools disabled
                    );

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


