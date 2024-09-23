use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::thread::{park, JoinHandle, Thread};
use vulkano::command_buffer::pool::CommandPool;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions};
use vulkano::swapchain::Surface;
use vulkano::VulkanLibrary;

use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};

use winit::error::EventLoopError;

use rendering::threading;

use futures::executor::block_on;
use futures::future::join;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::{WindowAttributes, WindowId};
use crate::rendering;
use crate::rendering::Position;

/**

    * 렌더링엔진입니다. 윈도우나 이벤트 헨들을 다른 쓰레드에서 처리하고 처리된 정보를 그저 실시간으로 읽어서 렌더링만 합니다.
    이런 방법을 통해서 렌더링에만 집중할 코드와 이벤트 처리를 담당할 부분을 분리할 수 있습니다.

    이를태면 렌더링할 모델을 정하거나 마우스 커서 움직임은 어플리케이션에서 담당하고 그걸 렌더링할 렌더러는 참조만 합니다.

*/
#[derive(Clone)]
pub struct Renderer {
    name: String,
    vulkan: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,

    core_count: u32,
    drawing: bool,

    //render_thread_pool: threading::RenderingThreadPool,

    render_object: Option<Arc<[Position]>>
}

/**

    * 현재는 멀티코어 렌더링을 계획했으나 기본적인 싱글코어 렌더링만 우선적으로 구현합니다.

*/
impl Renderer {
    pub fn new(/*event_loop: &EventLoop<()>*/surface_extension: InstanceExtensions, device_index: u32) -> Renderer {

        let vulkan_library = VulkanLibrary::new().expect("Failed to create vulkan library");


        let instance = Instance::new(
            vulkan_library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: surface_extension,
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

        let (device, mut queues) = Device::new(device.clone(), DeviceCreateInfo {
                queue_create_infos: vec![
                    QueueCreateInfo {
                        queue_family_index: graphics_queue_family_and_index.0 as u32,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }).unwrap();


        println!("Renderer created!");

        Renderer {
            name: "nefty renderer?".to_string(),
            vulkan: instance,
            device,
            queue: queues.next().unwrap(),

            core_count: 1,
            drawing: true,

            render_object: None,
        }
    }

    fn render_default(&self) {

    }
}



#[test]
fn test_render() {
    let main = thread::Builder::new().name("main".parse().unwrap()).spawn( || {

        struct App {}
        impl ApplicationHandler for App {
            fn resumed(&mut self, event_loop: &ActiveEventLoop) {
                todo!()
            }

            fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
                todo!()
            }
        }

        let mut event_loop = EventLoop::new().unwrap();
        let mut application = App {  };

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);

        let ext = Surface::required_extensions(&event_loop);

        let mut rnd = Renderer::new(ext, 0);

        let renderer = rendering::renderer::Renderer::new(ext, 0);

        event_loop.run_app(&mut application).expect("TODO: panic message");

    }).unwrap().join();
}