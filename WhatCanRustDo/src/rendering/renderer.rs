use std::fmt::Debug;
use std::sync::Arc;
use std::thread;
use std::thread::{park, JoinHandle, Thread};
use vulkano::command_buffer::pool::CommandPool;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use vulkano::VulkanLibrary;

use winit::event_loop::{EventLoop};

use winit::error::EventLoopError;
use winit::platform::windows::EventLoopBuilderExtWindows;

use futures::executor::block_on;

pub struct Position{
    x: f32,
    y: f32,
    z: f32
}

pub struct Mesh {
    positions: Vec<Position>
}




pub struct Renderer {
    name: String,
    vulkan: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,

    core_count: u32,
    drawing: bool,

    workers: Vec<JoinHandle<()>>,

    render_object: Option<Mesh>
}

impl Renderer {

    fn new(event_loop: Result<EventLoop<()>, EventLoopError>, device_index: u32) -> Renderer {
        let vulkan_library = VulkanLibrary::new().expect("Failed to create vulkan library");
        let instance_extensions = Surface::required_extensions(&event_loop.unwrap());


        let instance = Instance::new(
            vulkan_library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: instance_extensions,
                ..Default::default()
            }
        ).expect("failed to create Vulkan instance");

        let device = instance
            .enumerate_physical_devices()
            .expect("failed to get physical devices")
            .enumerate()
            .filter(|x| {
                x.0 == device_index as usize
            })
            .next()
            .expect("no such physical devices")
            .1;

        let graphics_queue_family_and_index = device
            .queue_family_properties()
            .iter()
            .enumerate()
            .find(|(x, y)| {
            y.queue_flags.contains(QueueFlags::GRAPHICS) && y.queue_flags.contains(QueueFlags::TRANSFER)
        }).expect("failed to get graphics queue family");

        let (device, mut queues) = if device.win32_presentation_support(graphics_queue_family_and_index.0 as u32).unwrap() {
            Device::new(device.clone(), DeviceCreateInfo {
                queue_create_infos: vec![
                    QueueCreateInfo {
                        queue_family_index: graphics_queue_family_and_index.0 as u32,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            })
        } else {
            panic!("Not a windows.");
        }.unwrap();

        Renderer {
            name: "nefty renderer?".to_string(),
            vulkan: instance,
            device,
            queue: queues.next().unwrap(),

            core_count: 1,
            drawing: true,
            workers: vec![],

            render_object: None,
        }
    }
}

#[test]
fn test_render() {
    let event_loop = EventLoop::builder().with_any_thread(true).build();

    let mut rnd = Renderer::new(event_loop, 0);
}