use futures::executor::block_on;
use imgui::*;
use imgui_wgpu::RendererConfig;
use imgui_winit_support;
use implot::{Colormap, ImPlotRange, Plot, PlotFlags, PlotLine};
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod domain;

const RUDA: &[u8] = include_bytes!("../Ruda-Bold.ttf");

fn main() {
    wgpu_subscriber::initialize_default_subscriber(None);

    // Set up window and GPU
    let event_loop = EventLoop::new();

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let (window, mut size, surface) = {
        let version = env!("CARGO_PKG_VERSION");

        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: 1280.0,
            height: 720.0,
        });
        window.set_title(&format!("imgui-wgpu {}", version));
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };

        (window, size, surface)
    };

    let mut hidpi_factor = window.scale_factor();

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }))
    .unwrap();

    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: false,
        },
        None,
    ))
    .unwrap();

    // Set up swap chain
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    let ruda = imgui.fonts().add_font(&[FontSource::TtfData {
        data: RUDA,
        size_pixels: font_size,
        config: None,
    }]);

    domain::style_ui(imgui.style_mut());

    let implot = implot::Context::create();

    //
    // Set up dear imgui wgpu renderer
    //
    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    #[cfg(not(feature = "glsl-to-spirv"))]
    let mut renderer = RendererConfig::new()
        .set_texture_format(sc_desc.format)
        .build(&mut imgui, &device, &queue);

    #[cfg(feature = "glsl-to-spirv")]
    let mut renderer = RendererConfig::new_glsl()
        .set_texture_format(sc_desc.format)
        .build(&mut imgui, &device, &queue);

    let mut last_frame = Instant::now();
    let start = last_frame.clone();

    let mut last_cursor = None;

    let mut xdata = vec![];
    let mut ydata = vec![];

    struct LogEntry {
        data: String,
        color: [f32; 4],
    }

    let mut log = vec![];
    let mut log_locked = false;

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                ..
            } => {
                hidpi_factor = scale_factor;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                size = window.inner_size();

                sc_desc = wgpu::SwapChainDescriptor {
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: wgpu::PresentMode::Mailbox,
                };

                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawEventsCleared => {
                let delta_s = last_frame.elapsed();
                let elapsed = start.elapsed().as_secs_f64();
                xdata.push(elapsed);
                ydata.push((elapsed * 10.0).sin());
                if log.len() % 10 == 0 {
                    log.push(LogEntry {
                        data: "Kek".to_string(),
                        color: [1.0, 0.0, 0.0, 1.0],
                    });
                } else if log.len() % 4 == 0 {
                    log.push(LogEntry {
                        data: "Top".to_string(),
                        color: [0.0, 1.0, 0.0, 1.0],
                    });
                } else {
                    log.push(LogEntry {
                        data: "Lorem Ipsum".to_string(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                }

                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;

                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {:?}", e);
                        return;
                    }
                };
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();
                let mut plot_ui = implot.get_plot_ui();

                let ruda = ui.push_font(ruda);
                let size = size.to_logical::<f32>(hidpi_factor);
                let canvas_size = PhysicalSize::new(size.width + 0.0, size.height - 20.0);

                {
                    ui.main_menu_bar(|| {
                        ui.text(im_str!("Frametime: {:?}", delta_s));
                    });

                    let window = imgui::Window::new(im_str!("Plot 1"));
                    window
                        .size(
                            [canvas_size.width * 0.75, canvas_size.height * 0.5],
                            Condition::Always,
                        )
                        .position([0.0, 0.0 + 20.0], Condition::Always)
                        .build(&ui, || {
                            let content_dimension = ui.content_region_avail();

                            let s = xdata.last().map(|v| *v).unwrap_or(0.0);

                            implot::set_colormap_from_preset(Colormap::Plasma, 1);
                            domain::style_seaborn();
                            Plot::new("Simple line plot")
                                // The size call could also be omitted, though the defaults don't consider window
                                // width, which is why we're not doing so here.
                                .size(content_dimension[0], content_dimension[1])
                                .y_limits(
                                    &ImPlotRange {
                                        Min: -1.0,
                                        Max: 1.0,
                                    },
                                    Condition::FirstUseEver,
                                )
                                .x_limits(
                                    &ImPlotRange {
                                        Min: s - 20.0,
                                        Max: s,
                                    },
                                    Condition::Always,
                                )
                                .with_plot_flags(&PlotFlags::ANTIALIASED)
                                .build(&mut plot_ui, || {
                                    PlotLine::new("legend label").plot(&xdata, &ydata);
                                });
                        });

                    let window = imgui::Window::new(im_str!("Plot 2"));
                    window
                        .size(
                            [canvas_size.width * 0.75, canvas_size.height * 0.5],
                            Condition::Always,
                        )
                        .position([0.0, canvas_size.height * 0.5 + 20.0], Condition::Always)
                        .build(&ui, || {
                            let content_dimension = ui.content_region_avail();

                            let s = xdata.last().map(|v| *v).unwrap_or(0.0);

                            implot::set_colormap_from_preset(Colormap::Plasma, 1);
                            domain::style_seaborn();
                            Plot::new("Simple line plot")
                                // The size call could also be omitted, though the defaults don't consider window
                                // width, which is why we're not doing so here.
                                .size(content_dimension[0], content_dimension[1])
                                .y_limits(
                                    &ImPlotRange {
                                        Min: -1.0,
                                        Max: 1.0,
                                    },
                                    Condition::FirstUseEver,
                                )
                                .x_limits(
                                    &ImPlotRange {
                                        Min: s - 20.0,
                                        Max: s,
                                    },
                                    Condition::Always,
                                )
                                .with_plot_flags(&PlotFlags::ANTIALIASED)
                                .build(&mut plot_ui, || {
                                    PlotLine::new("legend label").plot(&xdata, &ydata);
                                });
                        });

                    let window = imgui::Window::new(im_str!("Term"));
                    window
                        .size(
                            [canvas_size.width * 0.25, canvas_size.height],
                            Condition::Always,
                        )
                        .position([canvas_size.width * 0.75, 20.0], Condition::Always)
                        .build(&ui, || {
                            let content_dimension = ui.content_region_avail();
                            let log = &log;
                            TabBar::new(im_str!("Text Channel")).build(&ui, || {
                                if let Some(token) = TabItem::new(im_str!("Channel A")).begin(&ui) {
                                    ChildWindow::new(im_str!("Console1")).build(&ui, || {
                                        if ui.scroll_y() < ui.scroll_max_y() - 20.0 {
                                            log_locked = true;
                                        } else {
                                            log_locked = false;
                                        }
                                        if !log_locked {
                                            ui.set_scroll_y(ui.scroll_max_y());
                                        }
                                        for LogEntry { data, color } in log {
                                            ui.text_colored(*color, data);
                                        }
                                    });
                                    token.end(&ui);
                                }
                                if let Some(token) = TabItem::new(im_str!("Channel B")).begin(&ui) {
                                    token.end(&ui);
                                }
                            });
                        });

                    ruda.pop(&ui);
                }

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if last_cursor != Some(ui.mouse_cursor()) {
                    last_cursor = Some(ui.mouse_cursor());
                    platform.prepare_render(&ui, &window);
                }

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                renderer
                    .render(ui.render(), &queue, &device, &mut rpass)
                    .expect("Rendering failed");

                drop(rpass);

                queue.submit(Some(encoder.finish()));
            }
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}
