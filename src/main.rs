mod dom;
mod renderer;
mod view;
use dom::Dom;
use dom::element::Element;
use dom::debugtools::DebugTools;

use skia_safe::{AlphaType, Color, Color4f, ColorType, Font, FontMgr, FontStyle, ImageInfo, Paint, PaintStyle, Point, Rect, Surface, Typeface, surfaces};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use winit::window::Window;

use events::{EventSystem, MouseEvent, MouseEventType};

use crate::dom::events;
use crate::renderer::skiarenderer::SkiaRenderer;

#[path = "utils/winit_app.rs"]
mod winit_app;

#[cfg(not(target_os = "android"))]
fn main() {
    let s = String::from("hello");
    &s;
    entry(EventLoop::new().unwrap())
}

pub(crate) fn entry(event_loop: EventLoop<()>) {
    let mut debug_tools = DebugTools::new();
    let mut event_system = EventSystem::new();
    event_system.add_event_listener("mouseenter", |event: MouseEvent| {
        println!("Mouse entered node: {:?} at ({}, {})", event.node_id, event.x, event.y);
    });
    
    event_system.add_event_listener("mouseleave", |event: MouseEvent| {
        println!("Mouse left node: {:?} at ({}, {})", event.node_id, event.x, event.y);
    });
    let mut dom = Dom::new();
    
    // Create an element and add it to the DOM
    let element = Element::new();
    let root_node_id = dom.create_element(element);
    dom.set_root(root_node_id);
    
    // Create the view (now updated for new DOM structure)
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
                if let (PhysicalKey::Code(KeyCode::F12), ElementState::Pressed) = (event.physical_key, event.state) {
                    println!("F12 pressed!");
                    debug_tools.log("F12 pressed!");  // Uncommented
                    window.request_redraw();
                }
            }
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                
                let size = window.inner_size();
                if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    let mut buffer = surface.buffer_mut().unwrap();
                    
                    // Update the SkiaRenderer call to include debug tools
                    SkiaRenderer::render(
                        &mut dom,
                        buffer.as_mut(),
                        width.get() as usize,
                        height.get() as usize,
                        Some(&mut debug_tools),
                        Some(&mut event_system)
                        
                    );
                    
                    buffer.present().unwrap();
                }
            }
            Event::WindowEvent { window_id, event: WindowEvent::CursorMoved { position, .. } } if window_id == window.id() => {
                // Handle mouse movement
                let x = position.x as f32;
                let y = position.y as f32;
                
                // Process mouse movement for event system
                event_system.process_mouse_move(&dom, x, y);

                    window.request_redraw();
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