/*use std::sync::Arc;
use vulkano::device::Device;
use vulkano::instance::Instance;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, Pipeline};
use vulkano::sync::PipelineStage;

fn new_graphics_pipeline(dev: Arc<Device>, stages: PipelineStage ) -> Pipeline {


    GraphicsPipeline::new(dev.clone(), None,
                          GraphicsPipelineCreateInfo {
        // The stages of our pipeline, we have vertex and fragment stages.
        stages: stages.into_iter().collect(),
        // Describes the layout of the vertex input and how should it behave.
        vertex_input_state: Some(vertex_input_state),
        // Indicate the type of the primitives (the default is a list of triangles).
        input_assembly_state: Some(InputAssemblyState::default()),
        // Set the fixed viewport.
        viewport_state: Some(ViewportState {
          viewports: [viewport].into_iter().collect(),
          ..Default::default()
        }),
        // Ignore these for now.
        rasterization_state: Some(RasterizationState::default()),
        multisample_state: Some(MultisampleState::default()),
        color_blend_state: Some(ColorBlendState::with_attachment_states(
          subpass.num_color_attachments(),
          ColorBlendAttachmentState::default(),
        )),
        // This graphics pipeline object concerns the first pass of the render pass.
        subpass: Some(subpass.into()),
..GraphicsPipelineCreateInfo::layout(layout)
              }
    }).unwrap()
}
*/