/// 90% of the source code here is from
/// https://github.com/vulkano-rs/vulkano-examples/blob/master/src/bin/triangle.rs
use std::{
    sync::{Arc, Mutex},
    thread,
};

use swapchain::Surface;
use sync::{FlushError, GpuFuture};
use vulkano::{
    buffer::BufferUsage,
    buffer::CpuAccessibleBuffer,
    command_buffer::AutoCommandBufferBuilder,
    command_buffer::DynamicState,
    device::Device,
    device::DeviceExtensions,
    device::Queue,
    device::QueuesIter,
    framebuffer::Framebuffer,
    framebuffer::FramebufferAbstract,
    framebuffer::RenderPassAbstract,
    framebuffer::Subpass,
    image::ImageUsage,
    image::SwapchainImage,
    instance::{Instance, PhysicalDevice},
    pipeline::viewport::Viewport,
    pipeline::GraphicsPipeline,
    swapchain,
    swapchain::AcquireError,
    swapchain::ColorSpace,
    swapchain::FullscreenExclusive,
    swapchain::PresentMode,
    swapchain::SurfaceTransform,
    swapchain::Swapchain,
    swapchain::SwapchainCreationError,
    sync,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event, event::Event, event_loop::ControlFlow, event_loop::EventLoop, window::Window,
    window::WindowBuilder,
};

use super::graphic_engine::GraphicEngine;

/// Vertex struct
/// I use the macro impl_vertex! to tell Vulkano what are the inputs for the shader.
/// In this case, the inputs are 'position' and 'is_white'.
/// I will fill the screen with squares that can be turned off/on.
/// This is why I need the 'is_white' member for the vertices.
/// Unfortunately, the bool type is not allowed as input for shaders thus I use the u32 type.
/// Unfortunately, the u8 type is not allowed as input so yeah...
#[derive(Default, Debug, Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    is_white: u32,
}
vulkano::impl_vertex!(Vertex, position, is_white);

pub struct VulkanoInterface {
    event_loop: EventLoop<()>,
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

impl VulkanoInterface {
    pub fn new() -> VulkanoInterface {
        let instance = {
            let required_extensions = vulkano_win::required_extensions();

            Instance::new(None, &required_extensions, None).unwrap()
        };

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

        let event_loop = EventLoop::new();

        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, Arc::clone(&instance))
            .unwrap();

        let (device, mut queues) = {
            let queue_family = physical
                .queue_families()
                .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
                .unwrap();
            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
            };

            Device::new(
                physical,
                physical.supported_features(),
                &device_ext,
                [(queue_family, 0.5)].iter().cloned(),
            )
            .unwrap()
        };

        let queue = queues.next().unwrap();

        let (mut swapchain, images) = {
            let caps = surface.capabilities(physical).unwrap();

            Swapchain::new(
                Arc::clone(&device),
                Arc::clone(&surface),
                caps.min_image_count,
                caps.supported_formats[0].0,
                surface.window().inner_size().into(),
                1,
                ImageUsage::color_attachment(),
                &queue,
                SurfaceTransform::Identity,
                caps.supported_composite_alpha.iter().next().unwrap(),
                PresentMode::Fifo,
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            )
            .unwrap()
        };

        let mut previous_frame_end = Some(sync::now(Arc::clone(&device)).boxed());

        VulkanoInterface {
            event_loop,
            surface,
            device,
            queue,
            swapchain,
            images,
            previous_frame_end,
        }
    }
}

impl GraphicEngine for VulkanoInterface {
    fn clear_screen(&mut self) {
        todo!()
    }

    fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }

    fn is_running(&self) -> bool {
        todo!()
    }

    fn init(&mut self) {
        // To fill a 64*32 screen we need (64+1)*(32+1) vertices (I think)
        let (vertices, indexes) =
            get_square_vertices_and_indexes_from_screen(super::SCREEN_WIDTH, super::SCREEN_HEIGHT);

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            Arc::clone(&self.device),
            BufferUsage::all(),
            false,
            vertices.iter().cloned(),
        )
        .unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            Arc::clone(&self.device),
            BufferUsage::index_buffer(),
            false,
            indexes.iter().cloned(),
        )
        .unwrap();

        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                src: "
                    #version 450
    
                    layout(location = 0) in vec2 position;
                    layout(location = 1) in uint is_white;
                    layout(location = 0) out vec3 fragColor;

                    const vec3 WHITE = vec3(1., 1., 1.);
                    const vec3 BLACK = vec3(0., 0., 0.);
    
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        if(is_white > 0) {
                            fragColor = WHITE;
                        } else {
                            fragColor = BLACK;
                        }
                        
                    }
                "
            }
        }

        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                src: "
                    #version 450
    
                    layout(location = 0) in flat vec3 fragColor;
                    layout(location = 0) out vec4 f_color;
    
                    void main() {
                        f_color = vec4(fragColor, 1.0);
                    }
                "
            }
        }

        let vs = vs::Shader::load(Arc::clone(&self.device)).unwrap();
        let fs = fs::Shader::load(Arc::clone(&self.device)).unwrap();

        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(
                Arc::clone(&self.device),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: self.swapchain.format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(Arc::clone(&render_pass), 0).unwrap())
                .build(Arc::clone(&self.device))
                .unwrap(),
        );

        let mut dynamic_state = DynamicState::none();

        let mut framebuffers =
            window_size_dependent_setup(&self.images, render_pass.clone(), &mut dynamic_state);

        let mut recreate_swapchain = false;

        let swapchain = self.swapchain;
        let surface = self.surface;
        let device = self.device;
        let queue = self.queue;

        thread::spawn(move || {
            self.event_loop
                .run(move |event, _, control_flow| match event {
                    Event::WindowEvent {
                        event:
                            event::WindowEvent::MouseInput {
                                button: event::MouseButton::Left,
                                state: event::ElementState::Pressed,
                                ..
                            },
                        ..
                    } => { /*
                         previous_frame_end.as_mut().unwrap().cleanup_finished();
                         let mut vertices = vertex_buffer.write().unwrap();
                         println!("debut: {:?}", &*vertices);
                         vertices[3] = Vertex {
                             position: [-1.0, 0.],
                             is_on: 1,
                         };

                         vertices[4] = Vertex {
                             position: [0.0, 1.0],
                             is_on: 1,
                         };

                         vertices[5] = Vertex {
                             position: [1.0, 0.0],
                             is_on: 1,
                         };
                         println!("fin: {:?}", &*vertices);*/
                    }
                    Event::WindowEvent {
                        event: event::WindowEvent::CloseRequested,
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Event::WindowEvent {
                        event: event::WindowEvent::Resized(_),
                        ..
                    } => {
                        recreate_swapchain = true;
                    }
                    Event::RedrawEventsCleared => {
                        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

                        if recreate_swapchain {
                            let dimensions = surface.window().inner_size().into();
                            let (new_swapchain, new_images) =
                                match swapchain.recreate_with_dimensions(dimensions) {
                                    Ok(r) => r,
                                    Err(SwapchainCreationError::UnsupportedDimensions) => return,
                                    Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                                };
                            swapchain = new_swapchain;
                            framebuffers = window_size_dependent_setup(
                                &new_images,
                                render_pass.clone(),
                                &mut dynamic_state,
                            );
                            recreate_swapchain = false;
                        }

                        let (image_num, suboptimal, acquire_future) =
                            match swapchain::acquire_next_image(Arc::clone(&swapchain), None) {
                                Ok(r) => r,
                                Err(AcquireError::OutOfDate) => {
                                    recreate_swapchain = true;
                                    return;
                                }
                                Err(e) => panic!("Failed to acquire next image: {:?}", e),
                            };

                        if suboptimal {
                            recreate_swapchain = true;
                        }

                        let clear_values = vec![[0., 0., 0., 1.].into()];

                        let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
                            device.clone(),
                            queue.family(),
                        )
                        .unwrap();

                        builder
                            .begin_render_pass(
                                Arc::clone(&framebuffers[image_num]),
                                false,
                                clear_values,
                            )
                            .unwrap()
                            .draw_indexed(
                                Arc::clone(&pipeline),
                                &dynamic_state,
                                Arc::clone(&vertex_buffer),
                                Arc::clone(&index_buffer),
                                (),
                                (),
                            )
                            .unwrap()
                            .end_render_pass()
                            .unwrap();

                        let command_buffer = builder.build().unwrap();

                        let future = self
                            .previous_frame_end
                            .take()
                            .unwrap()
                            .join(acquire_future)
                            .then_execute(Arc::clone(&queue), command_buffer)
                            .unwrap()
                            .then_swapchain_present(
                                Arc::clone(&queue),
                                Arc::clone(&swapchain),
                                image_num,
                            )
                            .then_signal_fence_and_flush();

                        match future {
                            Ok(future) => {
                                self.previous_frame_end = Some(future.boxed());
                            }
                            Err(FlushError::OutOfDate) => {
                                recreate_swapchain = true;
                                self.previous_frame_end =
                                    Some(sync::now(Arc::clone(&device)).boxed());
                            }
                            Err(e) => {
                                println!("Failed to flush future: {:?}", e);
                                self.previous_frame_end =
                                    Some(sync::now(Arc::clone(&device)).boxed());
                            }
                        }
                    }
                    _ => (),
                });
        });
    }
}

// not mine
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

// mine
fn get_square_vertices_and_indexes_from_screen(width: u32, height: u32) -> (Vec<Vertex>, Vec<u32>) {
    const VULKAN_FRAME_WIDTH: f32 = 2.;
    const VULKAN_FRAME_HEIGHT: f32 = 2.;
    const VULKAN_WIDTH_OFFSET: f32 = -1.;
    const VULKAN_HEIGHT_OFFSET: f32 = -1.;
    const TRIANGLES_PER_SQUARE: usize = 2;
    const VERTICES_PER_TRIANGLE: usize = 3;

    let width_vertices = width + 1;
    let height_vertices = height + 1;

    let mut vertices = Vec::with_capacity((width_vertices * height_vertices) as usize);
    let mut indexes: Vec<u32> = Vec::with_capacity(
        width as usize * height as usize * TRIANGLES_PER_SQUARE * VERTICES_PER_TRIANGLE,
    );

    let step_width = VULKAN_FRAME_WIDTH / (width as f32);
    let step_height = VULKAN_FRAME_HEIGHT / (height as f32);

    for y in 0..height + 1 {
        for x in 0..width + 1 {
            vertices.push(Vertex {
                position: [
                    x as f32 * step_width + VULKAN_WIDTH_OFFSET,
                    y as f32 * step_height + VULKAN_HEIGHT_OFFSET,
                ],
                ..Vertex::default()
            });
        }
    }

    for j in 0..height {
        for i in 0..width {
            // top left half triangle
            indexes.push(i + j * width_vertices);
            indexes.push(i + j * width_vertices + 1);
            indexes.push(i + (j + 1) * width_vertices);
            // bottom right half triangle
            indexes.push(i + j * width_vertices + 1);
            indexes.push(i + (j + 1) * width_vertices);
            indexes.push(i + (j + 1) * width_vertices + 1);
        }
    }

    (vertices, indexes)
}
