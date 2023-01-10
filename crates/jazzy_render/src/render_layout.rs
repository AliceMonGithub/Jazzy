// use winit::{
//     event::*,
//     event_loop::{ControlFlow, EventLoop},
//     window::{Window, WindowBuilder},
// };

// struct RenderDescriptor {
//     backends: wgpu::Backends,
//     power_preference: wgpu::PowerPreference,

//     surface_descriptor: SurfaceDescriptor,
// }

// struct SurfaceDescriptor {
//     usage: wgpu::TextureUsages,
//     present_mode: wgpu::PresentMode,
//     alpha_mode: wgpu::CompositeAlphaMode,
// }

// struct RenderLayout {
//     surface: wgpu::Surface,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     config: wgpu::SurfaceConfiguration,
//     size: winit::dpi::PhysicalSize<u32>,

//     render_pipeline: wgpu::RenderPipeline,

//     vertex_buffer: wgpu::Buffer,
//     index_buffer: wgpu::Buffer,

//     indices_count: u32,
// }

// impl RenderLayout {
//     async fn new(window: &Window) -> Self {
//         let size = window.inner_size();
//         let instance = wgpu::Instance::new(wgpu::Backends::all());
//         let surface = unsafe { instance.create_surface(window) };
//         let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::default(),
//             compatible_surface: Some(&surface),
//             force_fallback_adapter: false,
//         }).await.unwrap();

//         let (device, queue) = adapter
//         .request_device(&wgpu::DeviceDescriptor {
//             label: Some("Request device"),
//             features: wgpu::Features::empty(),
//             limits: wgpu::Limits::default(),
//         }, None).await.unwrap();

//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface.get_supported_formats(&adapter)[0],
//             width: size.width,
//             height: size.height,
//             present_mode: wgpu::PresentMode::Fifo,
//             alpha_mode: wgpu::CompositeAlphaMode::Auto
//         };

//         surface.configure(&device, &config);

//         let render_pipeline_layout = device
//         .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("Render pipeline layout"),
//             bind_group_layouts: &[],
//             push_constant_ranges: &[],
//         });

//         let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: Some("Render pipeline"),
//             layout: Some(&render_pipeline_layout),
//             vertex: wgpu::VertexState {
                
//             }
//         })

//         Self {
//             surface,
//             device,
//             queue,
//             config,
//             size,
//         }
//     }
// }