use crate::{context::downcast_ref, SurfaceTargetUnsafe, UncapturedErrorHandler};

use std::{
    future::{ready, Ready},
    ops::Range,
    sync::Arc,
};

use wasi::webgpu::{
    graphics_context::GraphicsContext,
    mini_canvas::{CreateDesc, MiniCanvas},
    webgpu,
};

wit_bindgen::generate!({
    path: "../wit",
    world: "wgpu:backend/main",
});

impl From<crate::context::ObjectId> for () {
    fn from(_value: crate::context::ObjectId) -> Self {
        ()
    }
}
impl From<()> for crate::context::ObjectId {
    fn from(_value: ()) -> Self {
        crate::context::ObjectId::UNUSED
    }
}

#[derive(Debug)]
pub struct ContextWasiWebgpu(webgpu::Gpu);

impl crate::Context for ContextWasiWebgpu {
    type AdapterId = ();
    type AdapterData = webgpu::GpuAdapter;
    type DeviceId = ();
    type DeviceData = webgpu::GpuDevice;
    type QueueId = ();
    type QueueData = webgpu::GpuQueue;
    type ShaderModuleId = ();
    type ShaderModuleData = webgpu::GpuShaderModule;
    type BindGroupLayoutId = ();
    type BindGroupLayoutData = webgpu::GpuBindGroupLayout;
    type BindGroupId = ();
    type BindGroupData = webgpu::GpuBindGroup;
    type TextureViewId = ();
    type TextureViewData = webgpu::GpuTextureView;
    type SamplerId = ();
    type SamplerData = webgpu::GpuSampler;
    type BufferId = ();
    type BufferData = webgpu::GpuBuffer;
    type TextureId = ();
    type TextureData = webgpu::GpuTexture;
    type QuerySetId = ();
    type QuerySetData = webgpu::GpuQuerySet;
    type PipelineLayoutId = ();
    type PipelineLayoutData = webgpu::GpuPipelineLayout;
    type RenderPipelineId = ();
    type RenderPipelineData = webgpu::GpuRenderPipeline;
    type ComputePipelineId = ();
    type ComputePipelineData = webgpu::GpuComputePipeline;
    type CommandEncoderId = ();
    // Option so that command_encoder_finish can take ownership.
    type CommandEncoderData = Option<webgpu::GpuCommandEncoder>;
    type ComputePassId = ();
    // Option so that command_encoder_end_compute_pass can take ownership.
    type ComputePassData = Option<webgpu::GpuComputePassEncoder>;
    type RenderPassId = ();
    // Option so that command_encoder_end_render_pass can take ownership.
    type RenderPassData = Option<webgpu::GpuRenderPassEncoder>;
    type CommandBufferId = ();
    type CommandBufferData = webgpu::GpuCommandBuffer;
    type RenderBundleEncoderId = ();
    type RenderBundleEncoderData = webgpu::GpuRenderBundleEncoder;
    type RenderBundleId = ();
    type RenderBundleData = webgpu::GpuRenderBundle;

    type SurfaceId = ();
    type SurfaceData = (MiniCanvas, Arc<GraphicsContext>);
    type SurfaceOutputDetail = SurfaceOutputDetail;
    type SubmissionIndex = (); // TODO: fix type
    type SubmissionIndexData = (); // TODO: fix type

    type RequestAdapterFuture = Ready<Option<((), webgpu::GpuAdapter)>>;
    type RequestDeviceFuture = Ready<
        Result<
            (
                Self::DeviceId,
                Self::DeviceData,
                Self::QueueId,
                Self::QueueData,
            ),
            crate::RequestDeviceError,
        >,
    >;
    type PopErrorScopeFuture = Ready<Option<crate::Error>>; // TODO: fix type

    fn init(_instance_desc: wgt::InstanceDescriptor) -> Self {
        Self(webgpu::get_gpu())
    }

    unsafe fn instance_create_surface(
        &self,
        _target: SurfaceTargetUnsafe,
    ) -> Result<(Self::SurfaceId, Self::SurfaceData), crate::CreateSurfaceError> {
        let context = GraphicsContext::new();
        let canvas = MiniCanvas::new(CreateDesc {
            height: 100,
            width: 100,
            offscreen: false,
        });
        canvas.connect_graphics_context(&context);
        Ok(((), (canvas, Arc::new(context))))
    }

    fn instance_request_adapter(
        &self,
        _options: &crate::RequestAdapterOptions<'_, '_>,
    ) -> Self::RequestAdapterFuture {
        let adapter = self.0.request_adapter(None);
        ready(Some(((), adapter)))
    }

    fn adapter_request_device(
        &self,
        _adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
        _desc: &crate::DeviceDescriptor<'_>,
        _trace_dir: Option<&std::path::Path>,
    ) -> Self::RequestDeviceFuture {
        let device = adapter_data.request_device(None);
        let queue = device.queue();
        ready(Ok(((), device, (), queue)))
    }

    fn instance_poll_all_devices(&self, _force_wait: bool) -> bool {
        todo!()
    }

    fn adapter_is_surface_supported(
        &self,
        _adapter: &Self::AdapterId,
        _adapter_data: &Self::AdapterData,
        _surface: &Self::SurfaceId,
        _surface_data: &Self::SurfaceData,
    ) -> bool {
        todo!()
    }

    fn adapter_features(
        &self,
        _adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::Features {
        map_wgt_features(adapter_data.features())
    }

    fn adapter_limits(
        &self,
        _adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::Limits {
        map_wgt_limits(adapter_data.limits())
    }

    fn adapter_downlevel_capabilities(
        &self,
        _adapter: &Self::AdapterId,
        _adapter_data: &Self::AdapterData,
    ) -> wgt::DownlevelCapabilities {
        // WASI-WebGPU is assumed to be fully compliant
        wgt::DownlevelCapabilities::default()
    }

    fn adapter_get_info(
        &self,
        _adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::AdapterInfo {
        let _info = adapter_data.request_adapter_info();

        // TODO: use data from `request_adapter_info`
        wgt::AdapterInfo {
            name: String::new(),
            vendor: 0,
            device: 0,
            device_type: wgt::DeviceType::Other,
            driver: String::new(),
            driver_info: String::new(),
            backend: wgt::Backend::WasiWebGpu,
        }
    }

    fn adapter_get_texture_format_features(
        &self,
        _adapter: &Self::AdapterId,
        _adapter_data: &Self::AdapterData,
        _format: wgt::TextureFormat,
    ) -> wgt::TextureFormatFeatures {
        todo!()
    }

    fn adapter_get_presentation_timestamp(
        &self,
        _adapter: &Self::AdapterId,
        _adapter_data: &Self::AdapterData,
    ) -> wgt::PresentationTimestamp {
        todo!()
    }

    fn surface_get_capabilities(
        &self,
        _surface: &Self::SurfaceId,
        _surface_data: &Self::SurfaceData,
        _adapter: &Self::AdapterId,
        _adapter_data: &Self::AdapterData,
    ) -> wgt::SurfaceCapabilities {
        let mut formats = vec![
            wgt::TextureFormat::Rgba8Unorm,
            wgt::TextureFormat::Bgra8Unorm,
            wgt::TextureFormat::Rgba16Float,
        ];
        let mut mapped_formats = formats.iter().map(|format| {
            let format: webgpu::GpuTextureFormat = (*format).into();
            format
        });
        // Preferred canvas format will only be either "rgba8unorm" or "bgra8unorm".
        // https://www.w3.org/TR/webgpu/#dom-gpu-getpreferredcanvasformat
        let preferred_format = self.0.get_preferred_canvas_format();
        if let Some(index) = mapped_formats.position(|format| format == preferred_format) {
            formats.swap(0, index);
        }

        wgt::SurfaceCapabilities {
            // https://gpuweb.github.io/gpuweb/#supported-context-formats
            formats,
            // Doesn't really have meaning on the web.
            present_modes: vec![wgt::PresentMode::Fifo],
            alpha_modes: vec![wgt::CompositeAlphaMode::Opaque],
            // Statically set to RENDER_ATTACHMENT for now. See https://gpuweb.github.io/gpuweb/#dom-gpucanvasconfiguration-usage
            usages: wgt::TextureUsages::RENDER_ATTACHMENT,
        }
    }

    fn surface_configure(
        &self,
        _surface: &Self::SurfaceId,
        surface_data: &Self::SurfaceData,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        _config: &crate::SurfaceConfiguration,
    ) {
        device_data.connect_graphics_context(&surface_data.1);
    }

    fn surface_get_current_texture(
        &self,
        _surface: &Self::SurfaceId,
        surface_data: &Self::SurfaceData,
    ) -> (
        Option<Self::TextureId>,
        Option<Self::TextureData>,
        crate::SurfaceStatus,
        Self::SurfaceOutputDetail,
    ) {
        let (_canvas, context) = surface_data;
        let graphics_buffer = context.get_current_buffer();
        let texture = webgpu::GpuTexture::from_graphics_buffer(graphics_buffer);

        (
            Some(()),
            Some(texture),
            wgt::SurfaceStatus::Good,
            SurfaceOutputDetail {
                graphics_context: Arc::clone(&context),
            },
        )
    }

    fn surface_present(&self, _texture: &Self::TextureId, detail: &Self::SurfaceOutputDetail) {
        detail.graphics_context.present();
    }

    fn surface_texture_discard(
        &self,
        _texture: &Self::TextureId,
        _detail: &Self::SurfaceOutputDetail,
    ) {
        todo!()
    }

    fn device_features(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
    ) -> wgt::Features {
        map_wgt_features(device_data.features())
    }

    fn device_limits(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
    ) -> wgt::Limits {
        map_wgt_limits(device_data.limits())
    }

    fn device_downlevel_properties(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
    ) -> wgt::DownlevelCapabilities {
        todo!()
    }

    fn device_create_shader_module(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: crate::ShaderModuleDescriptor<'_>,
        _shader_bound_checks: wgt::ShaderBoundChecks,
    ) -> (Self::ShaderModuleId, Self::ShaderModuleData) {
        ((), device_data.create_shader_module(desc.into()))
    }

    unsafe fn device_create_shader_module_spirv(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _desc: &crate::ShaderModuleDescriptorSpirV<'_>,
    ) -> (Self::ShaderModuleId, Self::ShaderModuleData) {
        todo!()
    }

    fn device_create_bind_group_layout(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::BindGroupLayoutDescriptor<'_>,
    ) -> (Self::BindGroupLayoutId, Self::BindGroupLayoutData) {
        ((), device_data.create_bind_group_layout(&desc.into()))
    }

    fn device_create_bind_group(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::BindGroupDescriptor<'_>,
    ) -> (Self::BindGroupId, Self::BindGroupData) {
        ((), device_data.create_bind_group(desc.into()))
    }

    fn device_create_pipeline_layout(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::PipelineLayoutDescriptor<'_>,
    ) -> (Self::PipelineLayoutId, Self::PipelineLayoutData) {
        ((), device_data.create_pipeline_layout(&desc.into()))
    }

    fn device_create_render_pipeline(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::RenderPipelineDescriptor<'_>,
    ) -> (Self::RenderPipelineId, Self::RenderPipelineData) {
        ((), device_data.create_render_pipeline(&desc.into()))
    }

    fn device_create_compute_pipeline(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::ComputePipelineDescriptor<'_>,
    ) -> (Self::ComputePipelineId, Self::ComputePipelineData) {
        ((), device_data.create_compute_pipeline(&desc.into()))
    }

    fn device_create_buffer(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::BufferDescriptor<'_>,
    ) -> (Self::BufferId, Self::BufferData) {
        ((), device_data.create_buffer(&desc.into()))
    }

    fn device_create_texture(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::TextureDescriptor<'_>,
    ) -> (Self::TextureId, Self::TextureData) {
        ((), device_data.create_texture(&desc.into()))
    }

    fn device_create_sampler(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::SamplerDescriptor<'_>,
    ) -> (Self::SamplerId, Self::SamplerData) {
        ((), device_data.create_sampler(Some(&desc.into())))
    }

    fn device_create_query_set(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _desc: &crate::QuerySetDescriptor<'_>,
    ) -> (Self::QuerySetId, Self::QuerySetData) {
        todo!()
    }

    fn device_create_command_encoder(
        &self,
        _device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &crate::CommandEncoderDescriptor<'_>,
    ) -> (Self::CommandEncoderId, Self::CommandEncoderData) {
        (
            (),
            Some(device_data.create_command_encoder(Some(&desc.into()))),
        )
    }

    fn device_create_render_bundle_encoder(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _desc: &crate::RenderBundleEncoderDescriptor<'_>,
    ) -> (Self::RenderBundleEncoderId, Self::RenderBundleEncoderData) {
        todo!()
    }

    fn device_drop(&self, _device: &Self::DeviceId, _device_data: &Self::DeviceData) {
        // Dropped automatically
    }

    fn device_set_device_lost_callback(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _device_lost_callback: crate::context::DeviceLostCallback,
    ) {
        todo!()
    }

    fn device_destroy(&self, _device: &Self::DeviceId, _device_data: &Self::DeviceData) {
        todo!()
    }

    fn device_mark_lost(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _message: &str,
    ) {
        todo!()
    }

    fn queue_drop(&self, _queue: &Self::QueueId, _queue_data: &Self::QueueData) {
        // Dropped automatically
    }

    fn device_poll(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _maintain: crate::Maintain,
    ) -> wgt::MaintainResult {
        // Device is polled automatically
        crate::MaintainResult::SubmissionQueueEmpty
    }

    fn device_on_uncaptured_error(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _handler: Box<dyn UncapturedErrorHandler>,
    ) {
        todo!()
    }

    fn device_push_error_scope(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
        _filter: crate::ErrorFilter,
    ) {
        todo!()
    }

    fn device_pop_error_scope(
        &self,
        _device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
    ) -> Self::PopErrorScopeFuture {
        todo!()
    }

    fn buffer_map_async(
        &self,
        _buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        mode: crate::MapMode,
        range: Range<wgt::BufferAddress>,
        callback: crate::context::BufferMapCallback,
    ) {
        // TODO: make this function async once wasi can
        buffer_data.map_async(
            mode.into(),
            Some(range.start),
            Some(range.end - range.start),
        );
        (callback)(Ok(()));
    }

    fn buffer_get_mapped_range(
        &self,
        _buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        sub_range: Range<wgt::BufferAddress>,
    ) -> Box<dyn crate::context::BufferMappedRange> {
        let actual_mapping =
            buffer_data.get_mapped_range(Some(sub_range.start), Some(sub_range.end));
        let temporary_mapping = (0..actual_mapping.length())
            .map(|i| actual_mapping.get(i))
            .collect();
        Box::new(MappedBuffer {
            actual_mapping,
            temporary_mapping,
        })
    }

    fn buffer_unmap(&self, _buffer: &Self::BufferId, buffer_data: &Self::BufferData) {
        buffer_data.unmap()
    }

    fn texture_create_view(
        &self,
        _texture: &Self::TextureId,
        texture_data: &Self::TextureData,
        desc: &crate::TextureViewDescriptor<'_>,
    ) -> (Self::TextureViewId, Self::TextureViewData) {
        ((), texture_data.create_view(Some(&desc.into())))
    }

    fn surface_drop(&self, _surface: &Self::SurfaceId, _surface_data: &Self::SurfaceData) {
        // Dropped automatically
    }

    fn adapter_drop(&self, _adapter: &Self::AdapterId, _adapter_data: &Self::AdapterData) {
        // Dropped automatically
    }

    fn buffer_destroy(&self, _buffer: &Self::BufferId, _buffer_data: &Self::BufferData) {
        todo!()
    }

    fn buffer_drop(&self, _buffer: &Self::BufferId, _buffer_data: &Self::BufferData) {
        // Dropped automatically
    }

    fn texture_destroy(&self, _texture: &Self::TextureId, _texture_data: &Self::TextureData) {
        todo!()
    }

    fn texture_drop(&self, _texture: &Self::TextureId, _texture_data: &Self::TextureData) {
        // Dropped automatically
    }

    fn texture_view_drop(
        &self,
        _texture_view: &Self::TextureViewId,
        _texture_view_data: &Self::TextureViewData,
    ) {
        // Dropped automatically
    }

    fn sampler_drop(&self, _sampler: &Self::SamplerId, _sampler_data: &Self::SamplerData) {
        // Dropped automatically
    }

    fn query_set_drop(&self, _query_set: &Self::QuerySetId, _query_set_data: &Self::QuerySetData) {
        // Dropped automatically
    }

    fn bind_group_drop(
        &self,
        _bind_group: &Self::BindGroupId,
        _bind_group_data: &Self::BindGroupData,
    ) {
        // Dropped automatically
    }

    fn bind_group_layout_drop(
        &self,
        _bind_group_layout: &Self::BindGroupLayoutId,
        _bind_group_layout_data: &Self::BindGroupLayoutData,
    ) {
        // Dropped automatically
    }

    fn pipeline_layout_drop(
        &self,
        _pipeline_layout: &Self::PipelineLayoutId,
        _pipeline_layout_data: &Self::PipelineLayoutData,
    ) {
        // Dropped automatically
    }

    fn shader_module_drop(
        &self,
        _shader_module: &Self::ShaderModuleId,
        _shader_module_data: &Self::ShaderModuleData,
    ) {
        // Dropped automatically
    }

    fn command_encoder_drop(
        &self,
        _command_encoder: &Self::CommandEncoderId,
        _command_encoder_data: &Self::CommandEncoderData,
    ) {
        // Dropped automatically
    }

    fn command_buffer_drop(
        &self,
        _command_buffer: &Self::CommandBufferId,
        _command_buffer_data: &Self::CommandBufferData,
    ) {
        // Dropped automatically
    }

    fn render_bundle_drop(
        &self,
        _render_bundle: &Self::RenderBundleId,
        _render_bundle_data: &Self::RenderBundleData,
    ) {
        // Dropped automatically
    }

    fn compute_pipeline_drop(
        &self,
        _pipeline: &Self::ComputePipelineId,
        _pipeline_data: &Self::ComputePipelineData,
    ) {
        // Dropped automatically
    }

    fn render_pipeline_drop(
        &self,
        _pipeline: &Self::RenderPipelineId,
        _pipeline_data: &Self::RenderPipelineData,
    ) {
        // Dropped automatically
    }

    fn compute_pipeline_get_bind_group_layout(
        &self,
        _pipeline: &Self::ComputePipelineId,
        pipeline_data: &Self::ComputePipelineData,
        index: u32,
    ) -> (Self::BindGroupLayoutId, Self::BindGroupLayoutData) {
        ((), pipeline_data.get_bind_group_layout(index))
    }

    fn render_pipeline_get_bind_group_layout(
        &self,
        _pipeline: &Self::RenderPipelineId,
        _pipeline_data: &Self::RenderPipelineData,
        _index: u32,
    ) -> (Self::BindGroupLayoutId, Self::BindGroupLayoutData) {
        todo!()
    }

    fn command_encoder_copy_buffer_to_buffer(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        _source: &Self::BufferId,
        source_data: &Self::BufferData,
        source_offset: wgt::BufferAddress,
        _destination: &Self::BufferId,
        destination_data: &Self::BufferData,
        destination_offset: wgt::BufferAddress,
        copy_size: wgt::BufferAddress,
    ) {
        encoder_data.as_ref().unwrap().copy_buffer_to_buffer(
            source_data,
            source_offset,
            destination_data,
            destination_offset,
            copy_size,
        );
    }

    fn command_encoder_copy_buffer_to_texture(
        &self,
        _encoder: &Self::CommandEncoderId,
        _encoder_data: &Self::CommandEncoderData,
        _source: crate::ImageCopyBuffer<'_>,
        _destination: crate::ImageCopyTexture<'_>,
        _copy_size: wgt::Extent3d,
    ) {
        todo!()
    }

    fn command_encoder_copy_texture_to_buffer(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        source: crate::ImageCopyTexture<'_>,
        destination: crate::ImageCopyBuffer<'_>,
        copy_size: wgt::Extent3d,
    ) {
        encoder_data.as_ref().unwrap().copy_texture_to_buffer(
            &(&source).into(),
            &(&destination).into(),
            &copy_size.into(),
        );
    }

    fn command_encoder_copy_texture_to_texture(
        &self,
        _encoder: &Self::CommandEncoderId,
        _encoder_data: &Self::CommandEncoderData,
        _source: crate::ImageCopyTexture<'_>,
        _destination: crate::ImageCopyTexture<'_>,
        _copy_size: wgt::Extent3d,
    ) {
        todo!()
    }

    fn command_encoder_begin_compute_pass(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        desc: &crate::ComputePassDescriptor<'_>,
    ) -> (Self::ComputePassId, Self::ComputePassData) {
        (
            (),
            Some(
                encoder_data
                    .as_ref()
                    .unwrap()
                    .begin_compute_pass(Some(&desc.into())),
            ),
        )
    }

    fn command_encoder_end_compute_pass(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        _pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
    ) {
        webgpu::GpuComputePassEncoder::end(
            pass_data.take().unwrap(),
            encoder_data.as_ref().unwrap(),
        );
    }

    fn command_encoder_begin_render_pass(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        desc: &crate::RenderPassDescriptor<'_, '_>,
    ) -> (Self::RenderPassId, Self::RenderPassData) {
        let encoder_data = encoder_data.as_ref().unwrap();
        let render_pass = encoder_data.begin_render_pass(&desc.into());
        ((), Some(render_pass))
    }

    fn command_encoder_end_render_pass(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        _pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
    ) {
        webgpu::GpuRenderPassEncoder::end(
            pass_data.take().unwrap(),
            encoder_data.as_ref().unwrap(),
        );
    }

    fn command_encoder_finish(
        &self,
        _encoder: Self::CommandEncoderId,
        encoder_data: &mut Self::CommandEncoderData,
    ) -> (Self::CommandBufferId, Self::CommandBufferData) {
        let encoder_data = encoder_data.take().unwrap();
        let label = encoder_data.label();
        let desc = if label.is_empty() {
            None
        } else {
            Some(webgpu::GpuCommandBufferDescriptor {
                label: Some(encoder_data.label()),
            })
        };
        (
            (),
            webgpu::GpuCommandEncoder::finish(encoder_data, desc.as_ref()),
        )
    }

    fn command_encoder_clear_texture(
        &self,
        _encoder: &Self::CommandEncoderId,
        _encoder_data: &Self::CommandEncoderData,
        _texture: &crate::Texture,
        _subresource_range: &wgt::ImageSubresourceRange,
    ) {
        todo!()
    }

    fn command_encoder_clear_buffer(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        buffer: &crate::Buffer,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
    ) {
        encoder_data.as_ref().unwrap().clear_buffer(
            buffer.data.downcast_ref().unwrap(),
            Some(offset),
            size,
        );
    }

    fn command_encoder_insert_debug_marker(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        label: &str,
    ) {
        encoder_data.as_ref().unwrap().insert_debug_marker(label);
    }

    fn command_encoder_push_debug_group(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        label: &str,
    ) {
        encoder_data.as_ref().unwrap().push_debug_group(label);
    }

    fn command_encoder_pop_debug_group(
        &self,
        _encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
    ) {
        encoder_data.as_ref().unwrap().pop_debug_group();
    }

    fn command_encoder_write_timestamp(
        &self,
        _encoder: &Self::CommandEncoderId,
        _encoder_data: &Self::CommandEncoderData,
        _query_set: &Self::QuerySetId,
        _query_set_data: &Self::QuerySetData,
        _query_index: u32,
    ) {
        todo!()
    }

    fn command_encoder_resolve_query_set(
        &self,
        _encoder: &Self::CommandEncoderId,
        _encoder_data: &Self::CommandEncoderData,
        _query_set: &Self::QuerySetId,
        _query_set_data: &Self::QuerySetData,
        _first_query: u32,
        _query_count: u32,
        _destination: &Self::BufferId,
        _destination_data: &Self::BufferData,
        _destination_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_bundle_encoder_finish(
        &self,
        _encoder: Self::RenderBundleEncoderId,
        _encoder_data: Self::RenderBundleEncoderData,
        _desc: &crate::RenderBundleDescriptor<'_>,
    ) -> (Self::RenderBundleId, Self::RenderBundleData) {
        todo!()
    }

    fn queue_write_buffer(
        &self,
        _queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        _buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        offset: wgt::BufferAddress,
        data: &[u8],
    ) {
        queue_data.write_buffer(
            &buffer_data,
            offset as u64,
            None,
            data,
            Some(data.len() as u64),
        );
    }

    fn queue_validate_write_buffer(
        &self,
        _queue: &Self::QueueId,
        _queue_data: &Self::QueueData,
        _buffer: &Self::BufferId,
        _buffer_data: &Self::BufferData,
        _offset: wgt::BufferAddress,
        _size: wgt::BufferSize,
    ) -> Option<()> {
        todo!()
    }

    fn queue_create_staging_buffer(
        &self,
        _queue: &Self::QueueId,
        _queue_data: &Self::QueueData,
        _size: wgt::BufferSize,
    ) -> Option<Box<dyn crate::context::QueueWriteBuffer>> {
        todo!()
    }

    fn queue_write_staging_buffer(
        &self,
        _queue: &Self::QueueId,
        _queue_data: &Self::QueueData,
        _buffer: &Self::BufferId,
        _buffer_data: &Self::BufferData,
        _offset: wgt::BufferAddress,
        _staging_buffer: &dyn crate::context::QueueWriteBuffer,
    ) {
        todo!()
    }

    fn queue_write_texture(
        &self,
        _queue: &Self::QueueId,
        _queue_data: &Self::QueueData,
        _texture: crate::ImageCopyTexture<'_>,
        _data: &[u8],
        _data_layout: wgt::ImageDataLayout,
        _size: wgt::Extent3d,
    ) {
        todo!()
    }

    fn queue_submit<I: Iterator<Item = (Self::CommandBufferId, Self::CommandBufferData)>>(
        &self,
        _queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        command_buffers: I,
    ) -> (Self::SubmissionIndex, Self::SubmissionIndexData) {
        let command_buffers = command_buffers
            .map(|(_, command_buffer)| command_buffer)
            .collect();
        ((), queue_data.submit(command_buffers))
    }

    fn queue_get_timestamp_period(
        &self,
        _queue: &Self::QueueId,
        _queue_data: &Self::QueueData,
    ) -> f32 {
        todo!()
    }

    fn queue_on_submitted_work_done(
        &self,
        _queue: &Self::QueueId,
        _queue_data: &Self::QueueData,
        _callback: crate::context::SubmittedWorkDoneCallback,
    ) {
        todo!()
    }

    fn device_start_capture(&self, _device: &Self::DeviceId, _device_data: &Self::DeviceData) {
        todo!()
    }

    fn device_stop_capture(&self, _device: &Self::DeviceId, _device_data: &Self::DeviceData) {
        todo!()
    }

    fn compute_pass_set_pipeline(
        &self,
        _pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        _pipeline: &Self::ComputePipelineId,
        pipeline_data: &Self::ComputePipelineData,
    ) {
        pass_data.as_ref().unwrap().set_pipeline(pipeline_data);
    }

    fn compute_pass_set_bind_group(
        &self,
        _pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        index: u32,
        _bind_group: &Self::BindGroupId,
        bind_group_data: &Self::BindGroupData,
        offsets: &[wgt::DynamicOffset],
    ) {
        pass_data
            .as_ref()
            .unwrap()
            .set_bind_group(index, bind_group_data, Some(offsets));
    }

    fn compute_pass_set_push_constants(
        &self,
        _pass: &mut Self::ComputePassId,
        _pass_data: &mut Self::ComputePassData,
        _offset: u32,
        _data: &[u8],
    ) {
        todo!()
    }

    fn compute_pass_insert_debug_marker(
        &self,
        _pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        label: &str,
    ) {
        pass_data.as_ref().unwrap().insert_debug_marker(label);
    }

    fn compute_pass_push_debug_group(
        &self,
        _pass: &mut Self::ComputePassId,
        _pass_data: &mut Self::ComputePassData,
        _group_label: &str,
    ) {
        todo!()
    }

    fn compute_pass_pop_debug_group(
        &self,
        _pass: &mut Self::ComputePassId,
        _pass_data: &mut Self::ComputePassData,
    ) {
        todo!()
    }

    fn compute_pass_write_timestamp(
        &self,
        _pass: &mut Self::ComputePassId,
        _pass_data: &mut Self::ComputePassData,
        _query_set: &Self::QuerySetId,
        _query_set_data: &Self::QuerySetData,
        _query_index: u32,
    ) {
        todo!()
    }

    fn compute_pass_begin_pipeline_statistics_query(
        &self,
        _pass: &mut Self::ComputePassId,
        _pass_data: &mut Self::ComputePassData,
        _query_set: &Self::QuerySetId,
        _query_set_data: &Self::QuerySetData,
        _query_index: u32,
    ) {
        todo!()
    }

    fn compute_pass_end_pipeline_statistics_query(
        &self,
        _pass: &mut Self::ComputePassId,
        _pass_data: &mut Self::ComputePassData,
    ) {
        todo!()
    }

    fn compute_pass_dispatch_workgroups(
        &self,
        _pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        x: u32,
        y: u32,
        z: u32,
    ) {
        pass_data
            .as_ref()
            .unwrap()
            .dispatch_workgroups(x, Some(y), Some(z));
    }

    fn compute_pass_dispatch_workgroups_indirect(
        &self,
        _pass: &mut Self::ComputePassId,
        _pass_data: &mut Self::ComputePassData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_pipeline(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _pipeline: &Self::RenderPipelineId,
        _pipeline_data: &Self::RenderPipelineData,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_bind_group(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _index: u32,
        _bind_group: &Self::BindGroupId,
        _bind_group_data: &Self::BindGroupData,
        _offsets: &[wgt::DynamicOffset],
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_index_buffer(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _buffer: &Self::BufferId,
        _buffer_data: &Self::BufferData,
        _index_format: wgt::IndexFormat,
        _offset: wgt::BufferAddress,
        _size: Option<wgt::BufferSize>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_vertex_buffer(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _slot: u32,
        _buffer: &Self::BufferId,
        _buffer_data: &Self::BufferData,
        _offset: wgt::BufferAddress,
        _size: Option<wgt::BufferSize>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_push_constants(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _stages: wgt::ShaderStages,
        _offset: u32,
        _data: &[u8],
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _vertices: Range<u32>,
        _instances: Range<u32>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw_indexed(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _indices: Range<u32>,
        _base_vertex: i32,
        _instances: Range<u32>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw_indirect(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw_indexed_indirect(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indirect(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count: u32,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indexed_indirect(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count: u32,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indirect_count(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count_buffer: &Self::BufferId,
        _count_buffer_data: &Self::BufferData,
        _count_buffer_offset: wgt::BufferAddress,
        _max_count: u32,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indexed_indirect_count(
        &self,
        _encoder: &mut Self::RenderBundleEncoderId,
        _encoder_data: &mut Self::RenderBundleEncoderData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count_buffer: &Self::BufferId,
        _count_buffer_data: &Self::BufferData,
        _count_buffer_offset: wgt::BufferAddress,
        _max_count: u32,
    ) {
        todo!()
    }

    fn render_pass_set_pipeline(
        &self,
        _pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        _pipeline: &Self::RenderPipelineId,
        pipeline_data: &Self::RenderPipelineData,
    ) {
        pass_data.as_ref().unwrap().set_pipeline(pipeline_data);
    }

    fn render_pass_set_bind_group(
        &self,
        _pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        index: u32,
        _bind_group: &Self::BindGroupId,
        bind_group_data: &Self::BindGroupData,
        offsets: &[wgt::DynamicOffset],
    ) {
        pass_data
            .as_ref()
            .unwrap()
            .set_bind_group(index, bind_group_data, Some(offsets));
    }

    fn render_pass_set_index_buffer(
        &self,
        _pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        _buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        index_format: wgt::IndexFormat,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferSize>,
    ) {
        let size = match size {
            Some(size) => size.get(),
            None => buffer_data.size(),
        };
        pass_data.as_ref().unwrap().set_index_buffer(
            buffer_data,
            index_format.into(),
            offset,
            size,
        );
    }

    fn render_pass_set_vertex_buffer(
        &self,
        _pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        slot: u32,
        _buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferSize>,
    ) {
        let size = match size {
            Some(size) => size.get(),
            None => buffer_data.size(),
        };
        pass_data
            .as_ref()
            .unwrap()
            .set_vertex_buffer(slot, buffer_data, offset, size);
    }

    fn render_pass_set_push_constants(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _stages: wgt::ShaderStages,
        _offset: u32,
        _data: &[u8],
    ) {
        todo!()
    }

    fn render_pass_draw(
        &self,
        _pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        vertices: Range<u32>,
        instances: Range<u32>,
    ) {
        pass_data.as_ref().unwrap().draw(
            vertices.end - vertices.start,
            instances.end - instances.start,
            vertices.start,
            instances.start,
        );
    }

    fn render_pass_draw_indexed(
        &self,
        _pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) {
        pass_data.as_ref().unwrap().draw_indexed(
            indices.end - indices.start,
            instances.end - instances.start,
            indices.start,
            base_vertex,
            instances.start,
        );
    }

    fn render_pass_draw_indirect(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_pass_draw_indexed_indirect(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indirect(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count: u32,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indexed_indirect(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count: u32,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indirect_count(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count_buffer: &Self::BufferId,
        _count_buffer_data: &Self::BufferData,
        _count_buffer_offset: wgt::BufferAddress,
        _max_count: u32,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indexed_indirect_count(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _indirect_buffer: &Self::BufferId,
        _indirect_buffer_data: &Self::BufferData,
        _indirect_offset: wgt::BufferAddress,
        _count_buffer: &Self::BufferId,
        _count_buffer_data: &Self::BufferData,
        _count_buffer_offset: wgt::BufferAddress,
        _max_count: u32,
    ) {
        todo!()
    }

    fn render_pass_set_blend_constant(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _color: wgt::Color,
    ) {
        todo!()
    }

    fn render_pass_set_scissor_rect(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _x: u32,
        _y: u32,
        _width: u32,
        _height: u32,
    ) {
        todo!()
    }

    fn render_pass_set_viewport(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
        _min_depth: f32,
        _max_depth: f32,
    ) {
        todo!()
    }

    fn render_pass_set_stencil_reference(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _reference: u32,
    ) {
        todo!()
    }

    fn render_pass_insert_debug_marker(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _label: &str,
    ) {
        todo!()
    }

    fn render_pass_push_debug_group(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _group_label: &str,
    ) {
        todo!()
    }

    fn render_pass_pop_debug_group(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
    ) {
        todo!()
    }

    fn render_pass_write_timestamp(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _query_set: &Self::QuerySetId,
        _query_set_data: &Self::QuerySetData,
        _query_index: u32,
    ) {
        todo!()
    }

    fn render_pass_begin_occlusion_query(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _query_index: u32,
    ) {
        todo!()
    }

    fn render_pass_end_occlusion_query(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
    ) {
        todo!()
    }

    fn render_pass_begin_pipeline_statistics_query(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _query_set: &Self::QuerySetId,
        _query_set_data: &Self::QuerySetData,
        _query_index: u32,
    ) {
        todo!()
    }

    fn render_pass_end_pipeline_statistics_query(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
    ) {
        todo!()
    }

    fn render_pass_execute_bundles(
        &self,
        _pass: &mut Self::RenderPassId,
        _pass_data: &mut Self::RenderPassData,
        _render_bundles: &mut dyn Iterator<Item = (Self::RenderBundleId, &Self::RenderBundleData)>,
    ) {
        todo!()
    }
}

pub struct SurfaceOutputDetail {
    pub(crate) graphics_context: Arc<webgpu::GraphicsContext>,
}

// inefficient, since `temporary_mapping`` needs to be copied to `actual_mapping`.
// only needed becuase the `BufferMappedRange` trait methods return slices. If we change them to return `impl SliceIndex` we can get rid of this indirection.
#[derive(Debug)]
pub struct MappedBuffer {
    actual_mapping: webgpu::RemoteBuffer,
    temporary_mapping: Vec<u8>,
}

impl crate::context::BufferMappedRange for MappedBuffer {
    #[inline]
    fn slice(&self) -> &[u8] {
        &self.temporary_mapping
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [u8] {
        &mut self.temporary_mapping
    }
}

impl Drop for MappedBuffer {
    fn drop(&mut self) {
        // Copy from the temporary mapping back into the array buffer that was
        // originally provided by the runtime

        // TODO: use RemoteBuffer.set_range once available
        for (i, byte) in self.temporary_mapping.iter().enumerate() {
            self.actual_mapping.set(i as u32, *byte);
        }
    }
}

const FEATURES_MAPPING: [(wgt::Features, webgpu::GpuFeatureName, &str); 11] = [
    (
        wgt::Features::DEPTH_CLIP_CONTROL,
        webgpu::GpuFeatureName::DepthClipControl,
        "depth-clip-control",
    ),
    (
        wgt::Features::DEPTH32FLOAT_STENCIL8,
        webgpu::GpuFeatureName::Depth32floatStencil8,
        "depth32float-stencil8",
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_BC,
        webgpu::GpuFeatureName::TextureCompressionBc,
        "texture-compression-bc",
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_ETC2,
        webgpu::GpuFeatureName::TextureCompressionEtc2,
        "texture-compression-etc2",
    ),
    (
        wgt::Features::TEXTURE_COMPRESSION_ASTC,
        webgpu::GpuFeatureName::TextureCompressionAstc,
        "texture-compression-astc",
    ),
    (
        wgt::Features::TIMESTAMP_QUERY,
        webgpu::GpuFeatureName::TimestampQuery,
        "timestamp-query",
    ),
    (
        wgt::Features::INDIRECT_FIRST_INSTANCE,
        webgpu::GpuFeatureName::IndirectFirstInstance,
        "indirect-first-instance",
    ),
    (
        wgt::Features::SHADER_F16,
        webgpu::GpuFeatureName::ShaderF16,
        "shader-f16",
    ),
    (
        wgt::Features::RG11B10UFLOAT_RENDERABLE,
        webgpu::GpuFeatureName::Rg11b10ufloatRenderable,
        "rg11b10ufloat-renderable",
    ),
    (
        wgt::Features::BGRA8UNORM_STORAGE,
        webgpu::GpuFeatureName::Bgra8unormStorage,
        "bgra8unorm-storage",
    ),
    (
        wgt::Features::FLOAT32_FILTERABLE,
        webgpu::GpuFeatureName::Float32Filterable,
        "float32-filterable",
    ),
];

fn map_wgt_features(supported_features: webgpu::GpuSupportedFeatures) -> wgt::Features {
    let mut features = wgt::Features::empty();
    for (wgpu_feat, _, str_feat) in FEATURES_MAPPING {
        if supported_features.has(str_feat) {
            features |= wgpu_feat;
        }
    }
    features
}

fn map_wgt_limits(limits: webgpu::GpuSupportedLimits) -> wgt::Limits {
    wgt::Limits {
        max_texture_dimension_1d: limits.max_texture_dimension1_d(),
        max_texture_dimension_2d: limits.max_texture_dimension2_d(),
        max_texture_dimension_3d: limits.max_texture_dimension3_d(),
        max_texture_array_layers: limits.max_texture_array_layers(),
        max_bind_groups: limits.max_bind_groups(),
        max_bindings_per_bind_group: limits.max_bindings_per_bind_group(),
        max_dynamic_uniform_buffers_per_pipeline_layout: limits
            .max_dynamic_uniform_buffers_per_pipeline_layout(),
        max_dynamic_storage_buffers_per_pipeline_layout: limits
            .max_dynamic_storage_buffers_per_pipeline_layout(),
        max_sampled_textures_per_shader_stage: limits.max_sampled_textures_per_shader_stage(),
        max_samplers_per_shader_stage: limits.max_samplers_per_shader_stage(),
        max_storage_buffers_per_shader_stage: limits.max_storage_buffers_per_shader_stage(),
        max_storage_textures_per_shader_stage: limits.max_storage_textures_per_shader_stage(),
        max_uniform_buffers_per_shader_stage: limits.max_uniform_buffers_per_shader_stage(),
        max_uniform_buffer_binding_size: limits.max_uniform_buffer_binding_size() as u32,
        max_storage_buffer_binding_size: limits.max_storage_buffer_binding_size() as u32,
        max_vertex_buffers: limits.max_vertex_buffers(),
        max_buffer_size: limits.max_buffer_size() as u64,
        max_vertex_attributes: limits.max_vertex_attributes(),
        max_vertex_buffer_array_stride: limits.max_vertex_buffer_array_stride(),
        min_uniform_buffer_offset_alignment: limits.min_uniform_buffer_offset_alignment(),
        min_storage_buffer_offset_alignment: limits.min_storage_buffer_offset_alignment(),
        max_inter_stage_shader_components: limits.max_inter_stage_shader_components(),
        max_compute_workgroup_storage_size: limits.max_compute_workgroup_storage_size(),
        max_compute_invocations_per_workgroup: limits.max_compute_invocations_per_workgroup(),
        max_compute_workgroup_size_x: limits.max_compute_workgroup_size_x(),
        max_compute_workgroup_size_y: limits.max_compute_workgroup_size_y(),
        max_compute_workgroup_size_z: limits.max_compute_workgroup_size_z(),
        max_compute_workgroups_per_dimension: limits.max_compute_workgroups_per_dimension(),
        // The following are not part of WebGPU
        max_push_constant_size: wgt::Limits::default().max_push_constant_size,
        max_non_sampler_bindings: wgt::Limits::default().max_non_sampler_bindings,
    }
}

// type conversions

impl From<crate::VertexFormat> for webgpu::GpuVertexFormat {
    fn from(value: crate::VertexFormat) -> Self {
        match value {
            wgt::VertexFormat::Uint8x2 => webgpu::GpuVertexFormat::Uint8x2,
            wgt::VertexFormat::Uint8x4 => webgpu::GpuVertexFormat::Uint8x4,
            wgt::VertexFormat::Sint8x2 => webgpu::GpuVertexFormat::Sint8x2,
            wgt::VertexFormat::Sint8x4 => webgpu::GpuVertexFormat::Sint8x4,
            wgt::VertexFormat::Unorm8x2 => webgpu::GpuVertexFormat::Unorm8x2,
            wgt::VertexFormat::Unorm8x4 => webgpu::GpuVertexFormat::Unorm8x4,
            wgt::VertexFormat::Snorm8x2 => webgpu::GpuVertexFormat::Snorm8x2,
            wgt::VertexFormat::Snorm8x4 => webgpu::GpuVertexFormat::Snorm8x4,
            wgt::VertexFormat::Uint16x2 => webgpu::GpuVertexFormat::Uint16x2,
            wgt::VertexFormat::Uint16x4 => webgpu::GpuVertexFormat::Uint16x4,
            wgt::VertexFormat::Sint16x2 => webgpu::GpuVertexFormat::Sint16x2,
            wgt::VertexFormat::Sint16x4 => webgpu::GpuVertexFormat::Sint16x4,
            wgt::VertexFormat::Unorm16x2 => webgpu::GpuVertexFormat::Unorm16x2,
            wgt::VertexFormat::Unorm16x4 => webgpu::GpuVertexFormat::Unorm16x4,
            wgt::VertexFormat::Snorm16x2 => webgpu::GpuVertexFormat::Snorm16x2,
            wgt::VertexFormat::Snorm16x4 => webgpu::GpuVertexFormat::Snorm16x4,
            wgt::VertexFormat::Float16x2 => webgpu::GpuVertexFormat::Float16x2,
            wgt::VertexFormat::Float16x4 => webgpu::GpuVertexFormat::Float16x4,
            wgt::VertexFormat::Float32 => webgpu::GpuVertexFormat::Float32,
            wgt::VertexFormat::Float32x2 => webgpu::GpuVertexFormat::Float32x2,
            wgt::VertexFormat::Float32x3 => webgpu::GpuVertexFormat::Float32x3,
            wgt::VertexFormat::Float32x4 => webgpu::GpuVertexFormat::Float32x4,
            wgt::VertexFormat::Uint32 => webgpu::GpuVertexFormat::Uint32,
            wgt::VertexFormat::Uint32x2 => webgpu::GpuVertexFormat::Uint32x2,
            wgt::VertexFormat::Uint32x3 => webgpu::GpuVertexFormat::Uint32x3,
            wgt::VertexFormat::Uint32x4 => webgpu::GpuVertexFormat::Uint32x4,
            wgt::VertexFormat::Sint32 => webgpu::GpuVertexFormat::Sint32,
            wgt::VertexFormat::Sint32x2 => webgpu::GpuVertexFormat::Sint32x2,
            wgt::VertexFormat::Sint32x3 => webgpu::GpuVertexFormat::Sint32x3,
            wgt::VertexFormat::Sint32x4 => webgpu::GpuVertexFormat::Sint32x4,
            wgt::VertexFormat::Float64
            | wgt::VertexFormat::Float64x2
            | wgt::VertexFormat::Float64x3
            | wgt::VertexFormat::Float64x4 => {
                panic!("VERTEX_ATTRIBUTE_64BIT feature must be enabled to use Double formats")
            }
        }
    }
}

impl From<crate::VertexStepMode> for webgpu::GpuVertexStepMode {
    fn from(value: crate::VertexStepMode) -> Self {
        match value {
            wgt::VertexStepMode::Vertex => webgpu::GpuVertexStepMode::Vertex,
            wgt::VertexStepMode::Instance => webgpu::GpuVertexStepMode::Instance,
        }
    }
}

impl From<crate::FrontFace> for webgpu::GpuFrontFace {
    fn from(value: crate::FrontFace) -> Self {
        match value {
            wgt::FrontFace::Ccw => webgpu::GpuFrontFace::Ccw,
            wgt::FrontFace::Cw => webgpu::GpuFrontFace::Cw,
        }
    }
}

impl From<crate::PrimitiveTopology> for webgpu::GpuPrimitiveTopology {
    fn from(value: crate::PrimitiveTopology) -> Self {
        match value {
            wgt::PrimitiveTopology::PointList => webgpu::GpuPrimitiveTopology::PointList,
            wgt::PrimitiveTopology::LineList => webgpu::GpuPrimitiveTopology::LineList,
            wgt::PrimitiveTopology::LineStrip => webgpu::GpuPrimitiveTopology::LineStrip,
            wgt::PrimitiveTopology::TriangleList => webgpu::GpuPrimitiveTopology::TriangleList,
            wgt::PrimitiveTopology::TriangleStrip => webgpu::GpuPrimitiveTopology::TriangleStrip,
        }
    }
}

impl From<crate::IndexFormat> for webgpu::GpuIndexFormat {
    fn from(value: crate::IndexFormat) -> Self {
        match value {
            wgt::IndexFormat::Uint16 => webgpu::GpuIndexFormat::Uint16,
            wgt::IndexFormat::Uint32 => webgpu::GpuIndexFormat::Uint32,
        }
    }
}

impl From<crate::TextureDimension> for webgpu::GpuTextureDimension {
    fn from(value: crate::TextureDimension) -> Self {
        match value {
            wgt::TextureDimension::D1 => webgpu::GpuTextureDimension::OneD,
            wgt::TextureDimension::D2 => webgpu::GpuTextureDimension::TwoD,
            wgt::TextureDimension::D3 => webgpu::GpuTextureDimension::ThreeD,
        }
    }
}

impl From<crate::MapMode> for webgpu::GpuMapModeFlags {
    fn from(value: crate::MapMode) -> Self {
        match value {
            crate::MapMode::Read => 0,
            crate::MapMode::Write => 1,
        }
    }
}

impl From<crate::TextureFormat> for webgpu::GpuTextureFormat {
    fn from(value: crate::TextureFormat) -> Self {
        match value {
            wgt::TextureFormat::R8Unorm => webgpu::GpuTextureFormat::R8unorm,
            wgt::TextureFormat::R8Snorm => webgpu::GpuTextureFormat::R8snorm,
            wgt::TextureFormat::R8Uint => webgpu::GpuTextureFormat::R8uint,
            wgt::TextureFormat::R8Sint => webgpu::GpuTextureFormat::R8sint,
            wgt::TextureFormat::R16Uint => webgpu::GpuTextureFormat::R16uint,
            wgt::TextureFormat::R16Sint => webgpu::GpuTextureFormat::R16sint,
            wgt::TextureFormat::R16Float => webgpu::GpuTextureFormat::R16float,
            wgt::TextureFormat::Rg8Unorm => webgpu::GpuTextureFormat::Rg8unorm,
            wgt::TextureFormat::Rg8Snorm => webgpu::GpuTextureFormat::Rg8snorm,
            wgt::TextureFormat::Rg8Uint => webgpu::GpuTextureFormat::Rg8uint,
            wgt::TextureFormat::Rg8Sint => webgpu::GpuTextureFormat::Rg8sint,
            wgt::TextureFormat::R32Uint => webgpu::GpuTextureFormat::R32uint,
            wgt::TextureFormat::R32Sint => webgpu::GpuTextureFormat::R32sint,
            wgt::TextureFormat::R32Float => webgpu::GpuTextureFormat::R32float,
            wgt::TextureFormat::Rg16Uint => webgpu::GpuTextureFormat::Rg16uint,
            wgt::TextureFormat::Rg16Sint => webgpu::GpuTextureFormat::Rg16sint,
            wgt::TextureFormat::Rg16Float => webgpu::GpuTextureFormat::Rg16float,
            wgt::TextureFormat::Rgba8Unorm => webgpu::GpuTextureFormat::Rgba8unorm,
            wgt::TextureFormat::Rgba8UnormSrgb => webgpu::GpuTextureFormat::Rgba8unormSrgb,
            wgt::TextureFormat::Rgba8Snorm => webgpu::GpuTextureFormat::Rgba8snorm,
            wgt::TextureFormat::Rgba8Uint => webgpu::GpuTextureFormat::Rgba8uint,
            wgt::TextureFormat::Rgba8Sint => webgpu::GpuTextureFormat::Rgba8sint,
            wgt::TextureFormat::Bgra8Unorm => webgpu::GpuTextureFormat::Bgra8unorm,
            wgt::TextureFormat::Bgra8UnormSrgb => webgpu::GpuTextureFormat::Bgra8unormSrgb,
            wgt::TextureFormat::Rgb9e5Ufloat => webgpu::GpuTextureFormat::Rgb9e5ufloat,
            wgt::TextureFormat::Rgb10a2Uint => webgpu::GpuTextureFormat::Rgb10a2uint,
            wgt::TextureFormat::Rgb10a2Unorm => webgpu::GpuTextureFormat::Rgb10a2unorm,
            wgt::TextureFormat::Rg11b10Float => webgpu::GpuTextureFormat::Rg11b10ufloat,
            wgt::TextureFormat::Rg32Uint => webgpu::GpuTextureFormat::Rg32uint,
            wgt::TextureFormat::Rg32Sint => webgpu::GpuTextureFormat::Rg32sint,
            wgt::TextureFormat::Rg32Float => webgpu::GpuTextureFormat::Rg32float,
            wgt::TextureFormat::Rgba16Uint => webgpu::GpuTextureFormat::Rgba16uint,
            wgt::TextureFormat::Rgba16Sint => webgpu::GpuTextureFormat::Rgba16sint,
            wgt::TextureFormat::Rgba16Float => webgpu::GpuTextureFormat::Rgba16float,
            wgt::TextureFormat::Rgba32Uint => webgpu::GpuTextureFormat::Rgba32uint,
            wgt::TextureFormat::Rgba32Sint => webgpu::GpuTextureFormat::Rgba32sint,
            wgt::TextureFormat::Rgba32Float => webgpu::GpuTextureFormat::Rgba32float,
            wgt::TextureFormat::Stencil8 => webgpu::GpuTextureFormat::Stencil8,
            wgt::TextureFormat::Depth16Unorm => webgpu::GpuTextureFormat::Depth16unorm,
            wgt::TextureFormat::Depth24Plus => webgpu::GpuTextureFormat::Depth24plus,
            wgt::TextureFormat::Depth24PlusStencil8 => {
                webgpu::GpuTextureFormat::Depth24plusStencil8
            }
            wgt::TextureFormat::Depth32Float => webgpu::GpuTextureFormat::Depth32float,
            wgt::TextureFormat::Depth32FloatStencil8 => {
                webgpu::GpuTextureFormat::Depth32floatStencil8
            }
            wgt::TextureFormat::Bc1RgbaUnorm => webgpu::GpuTextureFormat::Bc1RgbaUnorm,
            wgt::TextureFormat::Bc1RgbaUnormSrgb => webgpu::GpuTextureFormat::Bc1RgbaUnormSrgb,
            wgt::TextureFormat::Bc2RgbaUnorm => webgpu::GpuTextureFormat::Bc2RgbaUnorm,
            wgt::TextureFormat::Bc2RgbaUnormSrgb => webgpu::GpuTextureFormat::Bc2RgbaUnormSrgb,
            wgt::TextureFormat::Bc3RgbaUnorm => webgpu::GpuTextureFormat::Bc3RgbaUnorm,
            wgt::TextureFormat::Bc3RgbaUnormSrgb => webgpu::GpuTextureFormat::Bc3RgbaUnormSrgb,
            wgt::TextureFormat::Bc4RUnorm => webgpu::GpuTextureFormat::Bc4RUnorm,
            wgt::TextureFormat::Bc4RSnorm => webgpu::GpuTextureFormat::Bc4RSnorm,
            wgt::TextureFormat::Bc5RgUnorm => webgpu::GpuTextureFormat::Bc5RgUnorm,
            wgt::TextureFormat::Bc5RgSnorm => webgpu::GpuTextureFormat::Bc5RgSnorm,
            wgt::TextureFormat::Bc6hRgbUfloat => webgpu::GpuTextureFormat::Bc6hRgbUfloat,
            wgt::TextureFormat::Bc6hRgbFloat => webgpu::GpuTextureFormat::Bc6hRgbFloat,
            wgt::TextureFormat::Bc7RgbaUnorm => webgpu::GpuTextureFormat::Bc7RgbaUnorm,
            wgt::TextureFormat::Bc7RgbaUnormSrgb => webgpu::GpuTextureFormat::Bc7RgbaUnormSrgb,
            wgt::TextureFormat::Etc2Rgb8Unorm => webgpu::GpuTextureFormat::Etc2Rgb8unorm,
            wgt::TextureFormat::Etc2Rgb8UnormSrgb => webgpu::GpuTextureFormat::Etc2Rgb8unormSrgb,
            wgt::TextureFormat::Etc2Rgb8A1Unorm => webgpu::GpuTextureFormat::Etc2Rgb8a1unorm,
            wgt::TextureFormat::Etc2Rgb8A1UnormSrgb => {
                webgpu::GpuTextureFormat::Etc2Rgb8a1unormSrgb
            }
            wgt::TextureFormat::Etc2Rgba8Unorm => webgpu::GpuTextureFormat::Etc2Rgba8unorm,
            wgt::TextureFormat::Etc2Rgba8UnormSrgb => webgpu::GpuTextureFormat::Etc2Rgba8unormSrgb,
            wgt::TextureFormat::EacR11Unorm => webgpu::GpuTextureFormat::EacR11unorm,
            wgt::TextureFormat::EacR11Snorm => webgpu::GpuTextureFormat::EacR11snorm,
            wgt::TextureFormat::EacRg11Unorm => webgpu::GpuTextureFormat::EacRg11unorm,
            wgt::TextureFormat::EacRg11Snorm => webgpu::GpuTextureFormat::EacRg11snorm,
            wgt::TextureFormat::Astc { block, channel } => match channel {
                wgt::AstcChannel::Unorm => match block {
                    wgt::AstcBlock::B4x4 => webgpu::GpuTextureFormat::Astc4x4Unorm,
                    wgt::AstcBlock::B5x4 => webgpu::GpuTextureFormat::Astc5x4Unorm,
                    wgt::AstcBlock::B5x5 => webgpu::GpuTextureFormat::Astc5x5Unorm,
                    wgt::AstcBlock::B6x5 => webgpu::GpuTextureFormat::Astc6x5Unorm,
                    wgt::AstcBlock::B6x6 => webgpu::GpuTextureFormat::Astc6x6Unorm,
                    wgt::AstcBlock::B8x5 => webgpu::GpuTextureFormat::Astc8x5Unorm,
                    wgt::AstcBlock::B8x6 => webgpu::GpuTextureFormat::Astc8x6Unorm,
                    wgt::AstcBlock::B8x8 => webgpu::GpuTextureFormat::Astc8x8Unorm,
                    wgt::AstcBlock::B10x5 => webgpu::GpuTextureFormat::Astc10x5Unorm,
                    wgt::AstcBlock::B10x6 => webgpu::GpuTextureFormat::Astc10x6Unorm,
                    wgt::AstcBlock::B10x8 => webgpu::GpuTextureFormat::Astc10x8Unorm,
                    wgt::AstcBlock::B10x10 => webgpu::GpuTextureFormat::Astc10x10Unorm,
                    wgt::AstcBlock::B12x10 => webgpu::GpuTextureFormat::Astc12x10Unorm,
                    wgt::AstcBlock::B12x12 => webgpu::GpuTextureFormat::Astc12x12Unorm,
                },
                wgt::AstcChannel::UnormSrgb => match block {
                    wgt::AstcBlock::B4x4 => webgpu::GpuTextureFormat::Astc4x4UnormSrgb,
                    wgt::AstcBlock::B5x4 => webgpu::GpuTextureFormat::Astc5x4UnormSrgb,
                    wgt::AstcBlock::B5x5 => webgpu::GpuTextureFormat::Astc5x5UnormSrgb,
                    wgt::AstcBlock::B6x5 => webgpu::GpuTextureFormat::Astc6x5UnormSrgb,
                    wgt::AstcBlock::B6x6 => webgpu::GpuTextureFormat::Astc6x6UnormSrgb,
                    wgt::AstcBlock::B8x5 => webgpu::GpuTextureFormat::Astc8x5UnormSrgb,
                    wgt::AstcBlock::B8x6 => webgpu::GpuTextureFormat::Astc8x6UnormSrgb,
                    wgt::AstcBlock::B8x8 => webgpu::GpuTextureFormat::Astc8x8UnormSrgb,
                    wgt::AstcBlock::B10x5 => webgpu::GpuTextureFormat::Astc10x5UnormSrgb,
                    wgt::AstcBlock::B10x6 => webgpu::GpuTextureFormat::Astc10x6UnormSrgb,
                    wgt::AstcBlock::B10x8 => webgpu::GpuTextureFormat::Astc10x8UnormSrgb,
                    wgt::AstcBlock::B10x10 => webgpu::GpuTextureFormat::Astc10x10UnormSrgb,
                    wgt::AstcBlock::B12x10 => webgpu::GpuTextureFormat::Astc12x10UnormSrgb,
                    wgt::AstcBlock::B12x12 => webgpu::GpuTextureFormat::Astc12x12UnormSrgb,
                },
                wgt::AstcChannel::Hdr => {
                    unimplemented!("Format {value:?} has no WebGPU equivilant")
                }
            },
            wgt::TextureFormat::R16Unorm
            | wgt::TextureFormat::R16Snorm
            | wgt::TextureFormat::Rg16Unorm
            | wgt::TextureFormat::Rg16Snorm
            | wgt::TextureFormat::Rgba16Unorm
            | wgt::TextureFormat::Rgba16Snorm
            | wgt::TextureFormat::NV12 => {
                unimplemented!("Format {value:?} has no WebGPU equivilant")
            }
        }
    }
}

impl From<webgpu::GpuTextureFormat> for crate::TextureFormat {
    fn from(value: webgpu::GpuTextureFormat) -> Self {
        match value {
            webgpu::GpuTextureFormat::R8unorm => wgt::TextureFormat::R8Unorm,
            webgpu::GpuTextureFormat::R8snorm => wgt::TextureFormat::R8Snorm,
            webgpu::GpuTextureFormat::R8uint => wgt::TextureFormat::R8Uint,
            webgpu::GpuTextureFormat::R8sint => wgt::TextureFormat::R8Sint,
            webgpu::GpuTextureFormat::R16uint => wgt::TextureFormat::R16Uint,
            webgpu::GpuTextureFormat::R16sint => wgt::TextureFormat::R16Sint,
            webgpu::GpuTextureFormat::R16float => wgt::TextureFormat::R16Float,
            webgpu::GpuTextureFormat::Rg8unorm => wgt::TextureFormat::Rg8Unorm,
            webgpu::GpuTextureFormat::Rg8snorm => wgt::TextureFormat::Rg8Snorm,
            webgpu::GpuTextureFormat::Rg8uint => wgt::TextureFormat::Rg8Uint,
            webgpu::GpuTextureFormat::Rg8sint => wgt::TextureFormat::Rg8Sint,
            webgpu::GpuTextureFormat::R32uint => wgt::TextureFormat::R32Uint,
            webgpu::GpuTextureFormat::R32sint => wgt::TextureFormat::R32Sint,
            webgpu::GpuTextureFormat::R32float => wgt::TextureFormat::R32Float,
            webgpu::GpuTextureFormat::Rg16uint => wgt::TextureFormat::Rg16Uint,
            webgpu::GpuTextureFormat::Rg16sint => wgt::TextureFormat::Rg16Sint,
            webgpu::GpuTextureFormat::Rg16float => wgt::TextureFormat::Rg16Float,
            webgpu::GpuTextureFormat::Rgba8unorm => wgt::TextureFormat::Rgba8Unorm,
            webgpu::GpuTextureFormat::Rgba8unormSrgb => wgt::TextureFormat::Rgba8UnormSrgb,
            webgpu::GpuTextureFormat::Rgba8snorm => wgt::TextureFormat::Rgba8Snorm,
            webgpu::GpuTextureFormat::Rgba8uint => wgt::TextureFormat::Rgba8Uint,
            webgpu::GpuTextureFormat::Rgba8sint => wgt::TextureFormat::Rgba8Sint,
            webgpu::GpuTextureFormat::Bgra8unorm => wgt::TextureFormat::Bgra8Unorm,
            webgpu::GpuTextureFormat::Bgra8unormSrgb => wgt::TextureFormat::Bgra8UnormSrgb,
            webgpu::GpuTextureFormat::Rgb9e5ufloat => wgt::TextureFormat::Rgb9e5Ufloat,
            webgpu::GpuTextureFormat::Rgb10a2uint => wgt::TextureFormat::Rgb10a2Uint,
            webgpu::GpuTextureFormat::Rgb10a2unorm => wgt::TextureFormat::Rgb10a2Unorm,
            webgpu::GpuTextureFormat::Rg11b10ufloat => wgt::TextureFormat::Rg11b10Float,
            webgpu::GpuTextureFormat::Rg32uint => wgt::TextureFormat::Rg32Uint,
            webgpu::GpuTextureFormat::Rg32sint => wgt::TextureFormat::Rg32Sint,
            webgpu::GpuTextureFormat::Rg32float => wgt::TextureFormat::Rg32Float,
            webgpu::GpuTextureFormat::Rgba16uint => wgt::TextureFormat::Rgba16Uint,
            webgpu::GpuTextureFormat::Rgba16sint => wgt::TextureFormat::Rgba16Sint,
            webgpu::GpuTextureFormat::Rgba16float => wgt::TextureFormat::Rgba16Float,
            webgpu::GpuTextureFormat::Rgba32uint => wgt::TextureFormat::Rgba32Uint,
            webgpu::GpuTextureFormat::Rgba32sint => wgt::TextureFormat::Rgba32Sint,
            webgpu::GpuTextureFormat::Rgba32float => wgt::TextureFormat::Rgba32Float,
            webgpu::GpuTextureFormat::Stencil8 => wgt::TextureFormat::Stencil8,
            webgpu::GpuTextureFormat::Depth16unorm => wgt::TextureFormat::Depth16Unorm,
            webgpu::GpuTextureFormat::Depth24plus => wgt::TextureFormat::Depth24Plus,
            webgpu::GpuTextureFormat::Depth24plusStencil8 => {
                wgt::TextureFormat::Depth24PlusStencil8
            }
            webgpu::GpuTextureFormat::Depth32float => wgt::TextureFormat::Depth32Float,
            webgpu::GpuTextureFormat::Depth32floatStencil8 => {
                wgt::TextureFormat::Depth32FloatStencil8
            }
            webgpu::GpuTextureFormat::Bc1RgbaUnorm => wgt::TextureFormat::Bc1RgbaUnorm,
            webgpu::GpuTextureFormat::Bc1RgbaUnormSrgb => wgt::TextureFormat::Bc1RgbaUnormSrgb,
            webgpu::GpuTextureFormat::Bc2RgbaUnorm => wgt::TextureFormat::Bc2RgbaUnorm,
            webgpu::GpuTextureFormat::Bc2RgbaUnormSrgb => wgt::TextureFormat::Bc2RgbaUnormSrgb,
            webgpu::GpuTextureFormat::Bc3RgbaUnorm => wgt::TextureFormat::Bc3RgbaUnorm,
            webgpu::GpuTextureFormat::Bc3RgbaUnormSrgb => wgt::TextureFormat::Bc3RgbaUnormSrgb,
            webgpu::GpuTextureFormat::Bc4RUnorm => wgt::TextureFormat::Bc4RUnorm,
            webgpu::GpuTextureFormat::Bc4RSnorm => wgt::TextureFormat::Bc4RSnorm,
            webgpu::GpuTextureFormat::Bc5RgUnorm => wgt::TextureFormat::Bc5RgUnorm,
            webgpu::GpuTextureFormat::Bc5RgSnorm => wgt::TextureFormat::Bc5RgSnorm,
            webgpu::GpuTextureFormat::Bc6hRgbUfloat => wgt::TextureFormat::Bc6hRgbUfloat,
            webgpu::GpuTextureFormat::Bc6hRgbFloat => wgt::TextureFormat::Bc6hRgbFloat,
            webgpu::GpuTextureFormat::Bc7RgbaUnorm => wgt::TextureFormat::Bc7RgbaUnorm,
            webgpu::GpuTextureFormat::Bc7RgbaUnormSrgb => wgt::TextureFormat::Bc7RgbaUnormSrgb,
            webgpu::GpuTextureFormat::Etc2Rgb8unorm => wgt::TextureFormat::Etc2Rgb8Unorm,
            webgpu::GpuTextureFormat::Etc2Rgb8unormSrgb => wgt::TextureFormat::Etc2Rgb8UnormSrgb,
            webgpu::GpuTextureFormat::Etc2Rgb8a1unorm => wgt::TextureFormat::Etc2Rgb8A1Unorm,
            webgpu::GpuTextureFormat::Etc2Rgb8a1unormSrgb => {
                wgt::TextureFormat::Etc2Rgb8A1UnormSrgb
            }
            webgpu::GpuTextureFormat::Etc2Rgba8unorm => wgt::TextureFormat::Etc2Rgba8Unorm,
            webgpu::GpuTextureFormat::Etc2Rgba8unormSrgb => wgt::TextureFormat::Etc2Rgba8UnormSrgb,
            webgpu::GpuTextureFormat::EacR11unorm => wgt::TextureFormat::EacR11Unorm,
            webgpu::GpuTextureFormat::EacR11snorm => wgt::TextureFormat::EacR11Snorm,
            webgpu::GpuTextureFormat::EacRg11unorm => wgt::TextureFormat::EacRg11Unorm,
            webgpu::GpuTextureFormat::EacRg11snorm => wgt::TextureFormat::EacRg11Snorm,
            webgpu::GpuTextureFormat::Astc4x4Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B4x4,
            },
            webgpu::GpuTextureFormat::Astc5x4Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B5x4,
            },
            webgpu::GpuTextureFormat::Astc5x5Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B5x5,
            },
            webgpu::GpuTextureFormat::Astc6x5Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B6x5,
            },
            webgpu::GpuTextureFormat::Astc6x6Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B6x6,
            },
            webgpu::GpuTextureFormat::Astc8x5Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B8x5,
            },
            webgpu::GpuTextureFormat::Astc8x6Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B8x6,
            },
            webgpu::GpuTextureFormat::Astc8x8Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B8x8,
            },
            webgpu::GpuTextureFormat::Astc10x5Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B10x5,
            },
            webgpu::GpuTextureFormat::Astc10x6Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B10x6,
            },
            webgpu::GpuTextureFormat::Astc10x8Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B10x8,
            },
            webgpu::GpuTextureFormat::Astc10x10Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B10x10,
            },
            webgpu::GpuTextureFormat::Astc12x10Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B12x10,
            },
            webgpu::GpuTextureFormat::Astc12x12Unorm => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::Unorm,
                block: wgt::AstcBlock::B12x12,
            },
            webgpu::GpuTextureFormat::Astc4x4UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B4x4,
            },
            webgpu::GpuTextureFormat::Astc5x4UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B5x4,
            },
            webgpu::GpuTextureFormat::Astc5x5UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B5x5,
            },
            webgpu::GpuTextureFormat::Astc6x5UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B6x5,
            },
            webgpu::GpuTextureFormat::Astc6x6UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B6x6,
            },
            webgpu::GpuTextureFormat::Astc8x5UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B8x5,
            },
            webgpu::GpuTextureFormat::Astc8x6UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B8x6,
            },
            webgpu::GpuTextureFormat::Astc8x8UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B8x8,
            },
            webgpu::GpuTextureFormat::Astc10x5UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B10x5,
            },
            webgpu::GpuTextureFormat::Astc10x6UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B10x6,
            },
            webgpu::GpuTextureFormat::Astc10x8UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B10x8,
            },
            webgpu::GpuTextureFormat::Astc10x10UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B10x10,
            },
            webgpu::GpuTextureFormat::Astc12x10UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B12x10,
            },
            webgpu::GpuTextureFormat::Astc12x12UnormSrgb => wgt::TextureFormat::Astc {
                channel: wgt::AstcChannel::UnormSrgb,
                block: wgt::AstcBlock::B12x12,
            },
        }
    }
}

impl From<crate::FilterMode> for webgpu::GpuFilterMode {
    fn from(value: crate::FilterMode) -> Self {
        match value {
            wgt::FilterMode::Nearest => webgpu::GpuFilterMode::Nearest,
            wgt::FilterMode::Linear => webgpu::GpuFilterMode::Linear,
        }
    }
}

impl From<crate::FilterMode> for webgpu::GpuMipmapFilterMode {
    fn from(value: crate::FilterMode) -> Self {
        match value {
            wgt::FilterMode::Nearest => webgpu::GpuMipmapFilterMode::Nearest,
            wgt::FilterMode::Linear => webgpu::GpuMipmapFilterMode::Linear,
        }
    }
}

impl From<crate::AddressMode> for webgpu::GpuAddressMode {
    fn from(value: crate::AddressMode) -> Self {
        match value {
            wgt::AddressMode::ClampToEdge => webgpu::GpuAddressMode::ClampToEdge,
            wgt::AddressMode::Repeat => webgpu::GpuAddressMode::Repeat,
            wgt::AddressMode::MirrorRepeat => webgpu::GpuAddressMode::MirrorRepeat,
            wgt::AddressMode::ClampToBorder => panic!("Clamp to border is not supported"),
        }
    }
}

impl From<crate::CompareFunction> for webgpu::GpuCompareFunction {
    fn from(value: crate::CompareFunction) -> Self {
        match value {
            wgt::CompareFunction::Never => webgpu::GpuCompareFunction::Never,
            wgt::CompareFunction::Less => webgpu::GpuCompareFunction::Less,
            wgt::CompareFunction::Equal => webgpu::GpuCompareFunction::Equal,
            wgt::CompareFunction::LessEqual => webgpu::GpuCompareFunction::LessEqual,
            wgt::CompareFunction::Greater => webgpu::GpuCompareFunction::Greater,
            wgt::CompareFunction::NotEqual => webgpu::GpuCompareFunction::NotEqual,
            wgt::CompareFunction::GreaterEqual => webgpu::GpuCompareFunction::GreaterEqual,
            wgt::CompareFunction::Always => webgpu::GpuCompareFunction::Always,
        }
    }
}

impl From<crate::SamplerBindingType> for webgpu::GpuSamplerBindingType {
    fn from(value: crate::SamplerBindingType) -> Self {
        match value {
            wgt::SamplerBindingType::Filtering => webgpu::GpuSamplerBindingType::Filtering,
            wgt::SamplerBindingType::NonFiltering => webgpu::GpuSamplerBindingType::NonFiltering,
            wgt::SamplerBindingType::Comparison => webgpu::GpuSamplerBindingType::Comparison,
        }
    }
}

impl From<crate::TextureSampleType> for webgpu::GpuTextureSampleType {
    fn from(value: crate::TextureSampleType) -> Self {
        match value {
            wgt::TextureSampleType::Float { filterable: true } => {
                webgpu::GpuTextureSampleType::Depth
            }
            wgt::TextureSampleType::Float { filterable: false } => {
                webgpu::GpuTextureSampleType::UnfilterableFloat
            }
            wgt::TextureSampleType::Depth => webgpu::GpuTextureSampleType::Depth,
            wgt::TextureSampleType::Sint => webgpu::GpuTextureSampleType::Sint,
            wgt::TextureSampleType::Uint => webgpu::GpuTextureSampleType::Uint,
        }
    }
}

impl From<crate::TextureViewDimension> for webgpu::GpuTextureViewDimension {
    fn from(value: crate::TextureViewDimension) -> Self {
        match value {
            wgt::TextureViewDimension::D1 => webgpu::GpuTextureViewDimension::OneD,
            wgt::TextureViewDimension::D2 => webgpu::GpuTextureViewDimension::TwoD,
            wgt::TextureViewDimension::D2Array => webgpu::GpuTextureViewDimension::TwoDArray,
            wgt::TextureViewDimension::Cube => webgpu::GpuTextureViewDimension::Cube,
            wgt::TextureViewDimension::CubeArray => webgpu::GpuTextureViewDimension::CubeArray,
            wgt::TextureViewDimension::D3 => webgpu::GpuTextureViewDimension::ThreeD,
        }
    }
}

impl From<crate::BlendFactor> for webgpu::GpuBlendFactor {
    fn from(value: crate::BlendFactor) -> Self {
        match value {
            wgt::BlendFactor::Zero => webgpu::GpuBlendFactor::Zero,
            wgt::BlendFactor::One => webgpu::GpuBlendFactor::One,
            wgt::BlendFactor::Src => webgpu::GpuBlendFactor::Src,
            wgt::BlendFactor::OneMinusSrc => webgpu::GpuBlendFactor::OneMinusSrc,
            wgt::BlendFactor::SrcAlpha => webgpu::GpuBlendFactor::SrcAlpha,
            wgt::BlendFactor::OneMinusSrcAlpha => webgpu::GpuBlendFactor::OneMinusSrcAlpha,
            wgt::BlendFactor::Dst => webgpu::GpuBlendFactor::Dst,
            wgt::BlendFactor::OneMinusDst => webgpu::GpuBlendFactor::OneMinusDst,
            wgt::BlendFactor::DstAlpha => webgpu::GpuBlendFactor::DstAlpha,
            wgt::BlendFactor::OneMinusDstAlpha => webgpu::GpuBlendFactor::OneMinusDstAlpha,
            wgt::BlendFactor::SrcAlphaSaturated => webgpu::GpuBlendFactor::SrcAlphaSaturated,
            wgt::BlendFactor::Constant => webgpu::GpuBlendFactor::Constant,
            wgt::BlendFactor::OneMinusConstant => webgpu::GpuBlendFactor::OneMinusConstant,
            wgt::BlendFactor::Src1
            | wgt::BlendFactor::OneMinusSrc1
            | wgt::BlendFactor::Src1Alpha
            | wgt::BlendFactor::OneMinusSrc1Alpha => {
                panic!(
                    "{:?} is not enabled for this backend",
                    wgt::Features::DUAL_SOURCE_BLENDING
                )
            }
        }
    }
}

impl From<crate::BlendOperation> for webgpu::GpuBlendOperation {
    fn from(value: crate::BlendOperation) -> Self {
        match value {
            wgt::BlendOperation::Add => webgpu::GpuBlendOperation::Add,
            wgt::BlendOperation::Subtract => webgpu::GpuBlendOperation::Subtract,
            wgt::BlendOperation::ReverseSubtract => webgpu::GpuBlendOperation::ReverseSubtract,
            wgt::BlendOperation::Min => webgpu::GpuBlendOperation::Min,
            wgt::BlendOperation::Max => webgpu::GpuBlendOperation::Max,
        }
    }
}

impl From<crate::TextureAspect> for webgpu::GpuTextureAspect {
    fn from(value: crate::TextureAspect) -> Self {
        match value {
            wgt::TextureAspect::All => webgpu::GpuTextureAspect::All,
            wgt::TextureAspect::StencilOnly => webgpu::GpuTextureAspect::StencilOnly,
            wgt::TextureAspect::DepthOnly => webgpu::GpuTextureAspect::DepthOnly,
            wgt::TextureAspect::Plane0
            | wgt::TextureAspect::Plane1
            | wgt::TextureAspect::Plane2 => {
                panic!("multi-plane textures are not supported")
            }
        }
    }
}

impl From<crate::StorageTextureAccess> for webgpu::GpuStorageTextureAccess {
    fn from(value: crate::StorageTextureAccess) -> Self {
        match value {
            wgt::StorageTextureAccess::WriteOnly => webgpu::GpuStorageTextureAccess::WriteOnly,
            wgt::StorageTextureAccess::ReadOnly => webgpu::GpuStorageTextureAccess::ReadOnly,
            wgt::StorageTextureAccess::ReadWrite => webgpu::GpuStorageTextureAccess::ReadWrite,
        }
    }
}

impl From<crate::StencilOperation> for webgpu::GpuStencilOperation {
    fn from(value: crate::StencilOperation) -> Self {
        match value {
            wgt::StencilOperation::Keep => webgpu::GpuStencilOperation::Keep,
            wgt::StencilOperation::Zero => webgpu::GpuStencilOperation::Zero,
            wgt::StencilOperation::Replace => webgpu::GpuStencilOperation::Replace,
            wgt::StencilOperation::Invert => webgpu::GpuStencilOperation::Invert,
            wgt::StencilOperation::IncrementClamp => webgpu::GpuStencilOperation::IncrementClamp,
            wgt::StencilOperation::DecrementClamp => webgpu::GpuStencilOperation::DecrementClamp,
            wgt::StencilOperation::IncrementWrap => webgpu::GpuStencilOperation::IncrementWrap,
            wgt::StencilOperation::DecrementWrap => webgpu::GpuStencilOperation::DecrementWrap,
        }
    }
}

impl From<crate::Face> for webgpu::GpuCullMode {
    fn from(value: crate::Face) -> Self {
        match value {
            wgt::Face::Front => webgpu::GpuCullMode::Front,
            wgt::Face::Back => webgpu::GpuCullMode::Back,
        }
    }
}

impl From<&crate::BlendComponent> for webgpu::GpuBlendComponent {
    fn from(value: &crate::BlendComponent) -> Self {
        Self {
            src_factor: Some(value.src_factor.into()),
            dst_factor: Some(value.dst_factor.into()),
            operation: Some(value.operation.into()),
        }
    }
}

impl From<&crate::BlendState> for webgpu::GpuBlendState {
    fn from(value: &crate::BlendState) -> Self {
        Self {
            color: (&value.color).into(),
            alpha: (&value.alpha).into(),
        }
    }
}

impl From<&crate::ColorTargetState> for webgpu::GpuColorTargetState {
    fn from(value: &crate::ColorTargetState) -> Self {
        Self {
            format: value.format.into(),
            blend: value.blend.map(|b| (&b).into()),
            write_mask: Some(value.write_mask.bits()),
        }
    }
}

impl<'a> From<&crate::FragmentState<'a>> for webgpu::GpuFragmentState<'a> {
    fn from(value: &crate::FragmentState<'a>) -> Self {
        Self {
            targets: value
                .targets
                .iter()
                .map(|t| t.as_ref().map(|t| t.into()))
                .collect(),
            module: downcast_ref(value.module.data.as_ref()),
            entry_point: value.entry_point.into(),
        }
    }
}

impl From<&crate::VertexAttribute> for webgpu::GpuVertexAttribute {
    fn from(value: &crate::VertexAttribute) -> Self {
        Self {
            format: value.format.into(),
            offset: value.offset,
            shader_location: value.shader_location,
        }
    }
}

impl<'a> From<&crate::VertexBufferLayout<'a>> for webgpu::GpuVertexBufferLayout {
    fn from(value: &crate::VertexBufferLayout<'a>) -> Self {
        Self {
            array_stride: value.array_stride,
            step_mode: Some(value.step_mode.into()),
            attributes: value.attributes.iter().map(|a| a.into()).collect(),
        }
    }
}

impl<'a> From<&crate::VertexState<'a>> for webgpu::GpuVertexState<'a> {
    fn from(value: &crate::VertexState<'a>) -> Self {
        Self {
            buffers: Some(value.buffers.iter().map(|b| b.into()).collect()),
            module: downcast_ref(value.module.data.as_ref()),
            entry_point: value.entry_point.into(),
        }
    }
}

impl From<&crate::PrimitiveState> for webgpu::GpuPrimitiveState {
    fn from(value: &crate::PrimitiveState) -> Self {
        Self {
            topology: Some(value.topology.into()),
            strip_index_format: value.strip_index_format.map(|s| s.into()),
            front_face: Some(value.front_face.into()),
            cull_mode: value.cull_mode.map(|cm| cm.into()),
            unclipped_depth: Some(value.unclipped_depth),
            // TODO: Handle `polygon_mode` and `conservative`
        }
    }
}

impl From<crate::Extent3d> for webgpu::GpuExtent3D {
    fn from(value: crate::Extent3d) -> Self {
        webgpu::GpuExtent3D::GpuExtent3DDict(webgpu::GpuExtent3DDict {
            width: value.width,
            height: Some(value.height),
            depth_or_array_layers: Some(value.depth_or_array_layers),
        })
    }
}

impl<'a> From<&crate::BufferBinding<'a>> for webgpu::GpuBufferBinding<'a> {
    fn from(value: &crate::BufferBinding<'a>) -> Self {
        Self {
            buffer: downcast_ref(value.buffer.data.as_ref()),
            offset: Some(value.offset),
            size: value.size.map(|s| s.try_into().unwrap()),
        }
    }
}

impl<'a> From<&crate::BindingResource<'a>> for webgpu::GpuBindingResource<'a> {
    fn from(value: &crate::BindingResource<'a>) -> Self {
        match value {
            crate::BindingResource::Buffer(buffer) => {
                webgpu::GpuBindingResource::GpuBufferBinding(buffer.into())
            }
            crate::BindingResource::BufferArray(_) => {
                panic!("WASI backend does not support arrays of buffers")
            }
            crate::BindingResource::Sampler(sampler) => {
                webgpu::GpuBindingResource::GpuSampler(downcast_ref(sampler.data.as_ref()))
            }
            crate::BindingResource::SamplerArray(_) => {
                panic!("WASI backend does not support arrays of samplers")
            }
            crate::BindingResource::TextureView(view) => {
                webgpu::GpuBindingResource::GpuTextureView(downcast_ref(view.data.as_ref()))
            }
            crate::BindingResource::TextureViewArray(_) => {
                panic!("WASI backend does not support BINDING_INDEXING extension")
            }
        }
    }
}

impl<'a> From<&crate::BindGroupEntry<'a>> for webgpu::GpuBindGroupEntry<'a> {
    fn from(value: &crate::BindGroupEntry<'a>) -> Self {
        webgpu::GpuBindGroupEntry {
            binding: value.binding,
            resource: (&value.resource).into(),
        }
    }
}

impl<'a> From<&crate::BindGroupLayoutEntry> for webgpu::GpuBindGroupLayoutEntry {
    fn from(entry: &crate::BindGroupLayoutEntry) -> Self {
        let mut mapped_entry = webgpu::GpuBindGroupLayoutEntry {
            binding: entry.binding,
            visibility: entry.visibility.bits(),
            buffer: None,
            sampler: None,
            texture: None,
            storage_texture: None,
            external_texture: None,
        };

        match entry.ty {
            wgt::BindingType::Buffer {
                ty,
                has_dynamic_offset,
                min_binding_size,
            } => {
                mapped_entry.buffer = Some(webgpu::GpuBufferBindingLayout {
                    type_: Some(match ty {
                        wgt::BufferBindingType::Uniform => webgpu::GpuBufferBindingType::Uniform,
                        wgt::BufferBindingType::Storage { read_only: true } => {
                            webgpu::GpuBufferBindingType::ReadOnlyStorage
                        }
                        wgt::BufferBindingType::Storage { read_only: false } => {
                            webgpu::GpuBufferBindingType::Storage
                        }
                    }),
                    has_dynamic_offset: Some(has_dynamic_offset),
                    min_binding_size: Some(min_binding_size.map(|i| i.try_into().unwrap()))
                        .unwrap(),
                });
            }
            wgt::BindingType::Sampler(sampler) => {
                mapped_entry.sampler = Some(webgpu::GpuSamplerBindingLayout {
                    type_: Some(sampler.into()),
                });
            }
            wgt::BindingType::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => {
                mapped_entry.texture = Some(webgpu::GpuTextureBindingLayout {
                    sample_type: Some(sample_type.into()),
                    view_dimension: view_dimension.into(),
                    multisampled: Some(multisampled.into()),
                });
            }
            wgt::BindingType::StorageTexture {
                access,
                format,
                view_dimension,
            } => {
                mapped_entry.storage_texture = Some(webgpu::GpuStorageTextureBindingLayout {
                    access: Some(access.into()),
                    format: format.into(),
                    view_dimension: view_dimension.into(),
                });
            }
            wgt::BindingType::AccelerationStructure => todo!(),
        }

        mapped_entry
    }
}

impl From<&crate::StencilFaceState> for webgpu::GpuStencilFaceState {
    fn from(value: &crate::StencilFaceState) -> Self {
        Self {
            compare: Some(value.compare.into()),
            fail_op: Some(value.fail_op.into()),
            depth_fail_op: Some(value.depth_fail_op.into()),
            pass_op: Some(value.pass_op.into()),
        }
    }
}

impl From<&crate::DepthStencilState> for webgpu::GpuDepthStencilState {
    fn from(value: &crate::DepthStencilState) -> Self {
        Self {
            format: value.format.into(),
            depth_write_enabled: Some(value.depth_write_enabled),
            depth_compare: Some(value.depth_compare.into()),
            stencil_front: Some((&value.stencil.front).into()),
            stencil_back: Some((&value.stencil.back).into()),
            stencil_read_mask: Some(value.stencil.read_mask),
            stencil_write_mask: Some(value.stencil.write_mask),
            depth_bias: Some(value.bias.constant),
            depth_bias_slope_scale: Some(value.bias.slope_scale),
            depth_bias_clamp: Some(value.bias.clamp),
        }
    }
}

impl From<&crate::MultisampleState> for webgpu::GpuMultisampleState {
    fn from(value: &crate::MultisampleState) -> Self {
        Self {
            count: Some(value.count.into()),
            mask: Some(value.mask as u32),
            alpha_to_coverage_enabled: Some(value.alpha_to_coverage_enabled),
        }
    }
}

impl<'a> From<&crate::ComputePassTimestampWrites<'a>>
    for webgpu::GpuComputePassTimestampWrites<'a>
{
    fn from(value: &crate::ComputePassTimestampWrites<'a>) -> Self {
        Self {
            query_set: downcast_ref(value.query_set.data.as_ref()),
            beginning_of_pass_write_index: value.beginning_of_pass_write_index,
            end_of_pass_write_index: value.end_of_pass_write_index,
        }
    }
}

impl<'a> From<&crate::ComputePassDescriptor<'a>> for webgpu::GpuComputePassDescriptor<'a> {
    fn from(value: &crate::ComputePassDescriptor<'a>) -> Self {
        Self {
            label: value.label.map(|l| l.into()),
            timestamp_writes: value.timestamp_writes.as_ref().map(|tw| tw.into()),
        }
    }
}

impl<'a> From<&crate::TextureViewDescriptor<'a>> for webgpu::GpuTextureViewDescriptor {
    fn from(value: &crate::TextureViewDescriptor<'a>) -> Self {
        Self {
            label: value.label.map(|l| l.into()),
            format: value.format.map(|f| f.into()),
            dimension: value.dimension.map(|d| d.into()),
            aspect: Some(value.aspect.into()),
            base_mip_level: Some(value.base_mip_level),
            mip_level_count: value.mip_level_count,
            base_array_layer: Some(value.base_array_layer),
            array_layer_count: value.array_layer_count,
        }
    }
}

impl<'a> From<&crate::CommandEncoderDescriptor<'a>> for webgpu::GpuCommandEncoderDescriptor {
    fn from(value: &crate::CommandEncoderDescriptor<'a>) -> Self {
        Self {
            label: value.label.map(|l| l.into()),
        }
    }
}

impl<'a> From<&crate::BufferDescriptor<'a>> for webgpu::GpuBufferDescriptor {
    fn from(value: &crate::BufferDescriptor<'a>) -> Self {
        webgpu::GpuBufferDescriptor {
            label: value.label.map(|l| l.into()),
            size: value.size,
            usage: value.usage.bits(),
            mapped_at_creation: Some(value.mapped_at_creation),
        }
    }
}

impl<'a> From<&crate::ComputePipelineDescriptor<'a>> for webgpu::GpuComputePipelineDescriptor<'a> {
    fn from(value: &crate::ComputePipelineDescriptor<'a>) -> Self {
        Self {
            compute: webgpu::GpuProgrammableStage {
                module: value.module.data.downcast_ref().unwrap(),
                entry_point: Some(value.entry_point.to_string()),
            },
            layout: match value.layout {
                Some(layout) => webgpu::GpuPipelineLayoutOrGpuAutoLayoutMode::GpuPipelineLayout(
                    layout.data.downcast_ref().unwrap(),
                ),
                None => webgpu::GpuPipelineLayoutOrGpuAutoLayoutMode::GpuAutoLayoutMode(
                    webgpu::GpuAutoLayoutMode::Auto,
                ),
            },
        }
    }
}

impl<'a> From<&crate::RenderPipelineDescriptor<'a>> for webgpu::GpuRenderPipelineDescriptor<'a> {
    fn from(value: &crate::RenderPipelineDescriptor<'a>) -> Self {
        Self {
            vertex: (&value.vertex).into(),
            primitive: Some((&value.primitive).into()),
            depth_stencil: value.depth_stencil.as_ref().map(|ds| ds.into()),
            multisample: Some((&value.multisample).into()),
            fragment: value.fragment.as_ref().map(|f| f.into()),
            layout: value.layout.map(|l| downcast_ref(l.data.as_ref())),
        }
    }
}

impl<'a> From<&crate::PipelineLayoutDescriptor<'a>> for webgpu::GpuPipelineLayoutDescriptor<'a> {
    fn from(value: &crate::PipelineLayoutDescriptor<'a>) -> Self {
        webgpu::GpuPipelineLayoutDescriptor {
            bind_group_layouts: value
                .bind_group_layouts
                .iter()
                .map(|b| downcast_ref(b.data.as_ref()))
                .collect(),
            label: value.label.map(|l| l.into()),
        }
    }
}

impl<'a> From<crate::ShaderModuleDescriptor<'a>> for webgpu::GpuShaderModuleDescriptor {
    fn from(value: crate::ShaderModuleDescriptor<'a>) -> Self {
        let source = match value.source {
            #[cfg(feature = "spirv")]
            crate::ShaderSource::SpirV(ref spv) => {
                use naga::{back, front, valid};

                let options = naga::front::spv::Options {
                    adjust_coordinate_space: false,
                    strict_capabilities: true,
                    block_ctx_dump_prefix: None,
                };
                let spv_parser = front::spv::Frontend::new(spv.iter().cloned(), &options);
                let spv_module = spv_parser.parse().unwrap();

                let mut validator = valid::Validator::new(
                    valid::ValidationFlags::all(),
                    valid::Capabilities::all(),
                );
                let spv_module_info = validator.validate(&spv_module).unwrap();

                let writer_flags = naga::back::wgsl::WriterFlags::empty();
                let wgsl_text =
                    back::wgsl::write_string(&spv_module, &spv_module_info, writer_flags).unwrap();
                wgsl_text
            }
            #[cfg(feature = "glsl")]
            crate::ShaderSource::Glsl {
                ref shader,
                stage,
                ref defines,
            } => {
                use naga::{back, front, valid};

                // Parse the given shader code and store its representation.
                let options = front::glsl::Options {
                    stage,
                    defines: defines.clone(),
                };
                let mut parser = front::glsl::Frontend::default();
                let glsl_module = parser.parse(&options, shader).unwrap();

                let mut validator = valid::Validator::new(
                    valid::ValidationFlags::all(),
                    valid::Capabilities::all(),
                );
                let glsl_module_info = validator.validate(&glsl_module).unwrap();

                let writer_flags = naga::back::wgsl::WriterFlags::empty();
                let wgsl_text =
                    back::wgsl::write_string(&glsl_module, &glsl_module_info, writer_flags)
                        .unwrap();
                wgsl_text
            }
            #[cfg(feature = "wgsl")]
            crate::ShaderSource::Wgsl(ref code) => code.to_string(),
            #[cfg(feature = "naga-ir")]
            crate::ShaderSource::Naga(module) => {
                use naga::{back, valid};

                let mut validator = valid::Validator::new(
                    valid::ValidationFlags::all(),
                    valid::Capabilities::all(),
                );
                let module_info = validator.validate(&module).unwrap();

                let writer_flags = naga::back::wgsl::WriterFlags::empty();
                let wgsl_text =
                    back::wgsl::write_string(&module, &module_info, writer_flags).unwrap();
                wgsl_text
            }
            crate::ShaderSource::Dummy(_) => {
                panic!("found `ShaderSource::Dummy`")
            }
        };
        Self {
            label: value.label.map(|l| l.into()),
            code: source,
            // TODO: pass the correct value
            compilation_hints: None,
        }
    }
}

impl<'a> From<&crate::TextureDescriptor<'a>> for webgpu::GpuTextureDescriptor {
    fn from(value: &crate::TextureDescriptor<'a>) -> Self {
        Self {
            label: value.label.map(|l| l.into()),
            size: value.size.into(),
            mip_level_count: Some(value.mip_level_count),
            sample_count: Some(value.sample_count),
            dimension: value.dimension.into(),
            format: value.format.into(),
            usage: value.usage.bits(),
            view_formats: Some(value.view_formats.iter().map(|f| (*f).into()).collect()),
        }
    }
}

impl<'a> From<&crate::SamplerDescriptor<'a>> for webgpu::GpuSamplerDescriptor {
    fn from(value: &crate::SamplerDescriptor<'a>) -> Self {
        webgpu::GpuSamplerDescriptor {
            label: value.label.map(|l| l.into()),
            address_mode_u: Some(value.address_mode_u.into()),
            address_mode_v: Some(value.address_mode_v.into()),
            address_mode_w: Some(value.address_mode_w.into()),
            mag_filter: Some(value.mag_filter.into()),
            min_filter: Some(value.min_filter.into()),
            mipmap_filter: Some(value.mipmap_filter.into()),
            lod_min_clamp: Some(value.lod_min_clamp),
            lod_max_clamp: Some(value.lod_max_clamp),
            compare: value.compare.map(|c| c.into()),
            max_anisotropy: None,
            // TODO: Handle `max_anisotropy`, `anisotropy_clamp`, and `border_color`.
        }
    }
}

impl<'a> From<&crate::BindGroupDescriptor<'a>> for webgpu::GpuBindGroupDescriptor<'a> {
    fn from(value: &crate::BindGroupDescriptor<'a>) -> Self {
        webgpu::GpuBindGroupDescriptor {
            layout: downcast_ref(value.layout.data.as_ref()),
            entries: value.entries.iter().map(|entry| entry.into()).collect(),
            label: value.label.map(|l| l.into()),
        }
    }
}

impl<'a> From<&crate::BindGroupLayoutDescriptor<'a>> for webgpu::GpuBindGroupLayoutDescriptor {
    fn from(value: &crate::BindGroupLayoutDescriptor<'a>) -> Self {
        webgpu::GpuBindGroupLayoutDescriptor {
            label: value.label.map(|l| l.into()),
            entries: value.entries.iter().map(|entry| entry.into()).collect(),
        }
    }
}

impl From<&crate::Color> for webgpu::GpuColor {
    fn from(value: &crate::Color) -> Self {
        webgpu::GpuColor::GpuColorDict(webgpu::GpuColorDict {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        })
    }
}

impl<V> From<&crate::LoadOp<V>> for webgpu::GpuLoadOp {
    fn from(value: &crate::LoadOp<V>) -> Self {
        match value {
            crate::LoadOp::Clear(_) => webgpu::GpuLoadOp::Clear,
            crate::LoadOp::Load => webgpu::GpuLoadOp::Load,
        }
    }
}

impl From<&crate::StoreOp> for webgpu::GpuStoreOp {
    fn from(value: &crate::StoreOp) -> Self {
        match value {
            crate::StoreOp::Store => webgpu::GpuStoreOp::Store,
            crate::StoreOp::Discard => webgpu::GpuStoreOp::Discard,
        }
    }
}

impl<'a> From<&crate::RenderPassColorAttachment<'a>> for webgpu::GpuRenderPassColorAttachment<'a> {
    fn from(value: &crate::RenderPassColorAttachment<'a>) -> Self {
        let clear_value = match &value.ops.load {
            crate::LoadOp::Clear(color) => Some(color.into()),
            crate::LoadOp::Load => None,
        };
        webgpu::GpuRenderPassColorAttachment {
            view: value.view.data.as_ref().downcast_ref().unwrap(),
            resolve_target: value.resolve_target.map(|t| t.data.downcast_ref().unwrap()),
            clear_value,
            load_op: (&value.ops.load).into(),
            store_op: (&value.ops.store).into(),
            depth_slice: None,
        }
    }
}

impl<'a> From<&crate::RenderPassDepthStencilAttachment<'a>>
    for webgpu::GpuRenderPassDepthStencilAttachment<'a>
{
    fn from(value: &crate::RenderPassDepthStencilAttachment<'a>) -> Self {
        let depth_clear_value = value
            .depth_ops
            .as_ref()
            .map(|o| match o.load {
                crate::LoadOp::Clear(color) => Some(color.into()),
                crate::LoadOp::Load => None,
            })
            .flatten();
        let stencil_clear_value = value
            .stencil_ops
            .as_ref()
            .map(|o| match o.load {
                crate::LoadOp::Clear(color) => Some(color.into()),
                crate::LoadOp::Load => None,
            })
            .flatten();
        webgpu::GpuRenderPassDepthStencilAttachment {
            view: value.view.data.as_ref().downcast_ref().unwrap(),
            depth_clear_value,
            depth_load_op: value.depth_ops.as_ref().map(|o| (&o.load).into()),
            depth_store_op: value.depth_ops.as_ref().map(|o| (&o.store).into()),
            depth_read_only: Some(value.depth_ops.is_none()),
            stencil_clear_value,
            stencil_load_op: value.stencil_ops.as_ref().map(|o| (&o.load).into()),
            stencil_store_op: value.stencil_ops.as_ref().map(|o| (&o.store).into()),
            stencil_read_only: Some(value.stencil_ops.is_none()),
        }
    }
}

impl<'a> From<&crate::RenderPassTimestampWrites<'a>> for webgpu::GpuRenderPassTimestampWrites<'a> {
    fn from(value: &crate::RenderPassTimestampWrites<'a>) -> Self {
        webgpu::GpuRenderPassTimestampWrites {
            query_set: value.query_set.data.downcast_ref().unwrap(),
            beginning_of_pass_write_index: value.beginning_of_pass_write_index,
            end_of_pass_write_index: value.end_of_pass_write_index,
        }
    }
}

impl<'a> From<&crate::RenderPassDescriptor<'a, 'a>> for webgpu::GpuRenderPassDescriptor<'a> {
    fn from(value: &crate::RenderPassDescriptor<'a, 'a>) -> Self {
        webgpu::GpuRenderPassDescriptor {
            color_attachments: value
                .color_attachments
                .iter()
                .map(|ca| {
                    ca.as_ref()
                        .expect("TODO: will sleve resolve with updated wit")
                        .into()
                })
                .collect(),
            depth_stencil_attachment: value.depth_stencil_attachment.as_ref().map(|a| a.into()),
            occlusion_query_set: value
                .occlusion_query_set
                .as_ref()
                .map(|qs| qs.data.downcast_ref().unwrap()),
            timestamp_writes: value.timestamp_writes.as_ref().map(|t| t.into()),
            label: value.label.map(|l| l.into()),
            max_draw_count: None,
        }
    }
}

impl<'a> From<&crate::ImageCopyTexture<'a>> for webgpu::GpuImageCopyTexture<'a> {
    fn from(value: &crate::ImageCopyTexture<'a>) -> Self {
        webgpu::GpuImageCopyTexture {
            texture: value.texture.data.downcast_ref().unwrap(),
            mip_level: Some(value.mip_level),
            origin: Some((&value.origin).into()),
            aspect: Some(value.aspect.into()),
        }
    }
}

impl<'a> From<&crate::ImageCopyBuffer<'a>> for webgpu::GpuImageCopyBuffer<'a> {
    fn from(value: &crate::ImageCopyBuffer<'a>) -> Self {
        webgpu::GpuImageCopyBuffer {
            buffer: value.buffer.data.downcast_ref().unwrap(),
            offset: Some(value.layout.offset),
            bytes_per_row: value.layout.bytes_per_row,
            rows_per_image: value.layout.rows_per_image,
        }
    }
}

impl From<&crate::Origin3d> for webgpu::GpuOrigin3DDictOrListGpuIntegerCoordinate {
    fn from(value: &crate::Origin3d) -> Self {
        webgpu::GpuOrigin3DDictOrListGpuIntegerCoordinate::GpuOrigin3DDict(
            webgpu::GpuOrigin3DDict {
                x: Some(value.x),
                y: Some(value.y),
                z: Some(value.z),
            },
        )
    }
}
