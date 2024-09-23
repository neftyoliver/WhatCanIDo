use std::sync::Arc;
use vulkano::device::Device;
use vulkano::instance::Instance;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, Pipeline};
use vulkano::sync::PipelineStage;




/*pub mod pipeline {
    use std::sync::Arc;
    use vulkano::device::Device;
    use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
    use vulkano::pipeline::GraphicsPipeline;
    use vulkano::sync::PipelineStage;

    pub mod stages {

    }

    pub fn new_graphics_pipeline(dev: Arc<Device>, stages: PipelineStage) -> Arc<GraphicsPipeline> {
        let pipeline_stages = {

        };

        GraphicsPipeline::new(
            dev.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: Default::default(),
                vertex_input_state: None,
                input_assembly_state: None,
                tessellation_state: None,
                viewport_state: None,
                rasterization_state: None,
                multisample_state: None,
                depth_stencil_state: None,
                color_blend_state: None,
                dynamic_state: Default::default(),
                layout: Default::default(),
                subpass: None,
                base_pipeline: None,
                discard_rectangle_state: None,
                ..Default::default()
            }).unwrap()
    }
}*/