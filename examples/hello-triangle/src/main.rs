use std::mem::{MaybeUninit, size_of, size_of_val, transmute};
use std::num::NonZeroUsize;
use std::panic::catch_unwind;
use std::pin::Pin;
use std::ptr::{copy_nonoverlapping};
use array_init::try_array_init;
use d3d12_api::aliases::win32::graphics::direct3d12::{Blend, BlendDesc, BlendOp, ColorWriteEnable, CommandAllocator, CommandList, CommandListType, CommandQueue, CommandQueueDesc, CpuDescriptorHandle, CullMode, Debug, DescriptorHeap, DescriptorHeapDesc, DescriptorHeapFlags, DescriptorHeapType, Device, Feature, FeatureDataRootSignature, Fence, FenceFlags, FillMode, GraphicsCommandList, GraphicsPipelineStateDesc, HeapFlags, HeapProperties, InputClassification, InputElementDesc, InputLayoutDesc, LogicOp, PipelineState, PrimitiveTopologyType, RasterizerDesc, RenderTargetBlendDesc, Resource, ResourceDesc, ResourceStates, RootSignature, RootSignatureVersion, ShaderByteCode, VertexBufferView, Viewport};
use d3d12_api::aliases::win32::graphics::direct3d::FeatureLevel;
use d3d12_api::aliases::win32::graphics::dxgi::{Adapter1, AdapterFlag, Factory4, InfoQueue, Present, Scaling, SwapChain3, SwapChainDesc1, SwapEffect, Usage};
use d3d12_api::aliases::win32::graphics::dxgi::common::{AlphaMode, Format, SampleDesc};
use d3d12_api::core::win32::foundation::{AsPStr, Handle, HResult, HWnd, LParam, LResult, OkOrErr, Rect, Win32Error, WParam};
use d3d12_api::core::win32::graphics::direct3d12::{D3D12CreateDevice, D3D12GetDebugInterface, ID3D12RootSignature};
use d3d12_api::core::win32::graphics::direct3d::D3DPrimitiveTopology;
use d3d12_api::core::win32::graphics::dxgi::{CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG, DxgiFactory1, DxgiFactory2, DxgiFactory4, DXGIGetDebugInterface1, DxgiMwa, DxgiSwapChainFlag};
use d3d12_api::core::win32::system::com::{AsParam, IUnknown};
use d3d12_api::core::win32::system::library_loader::GetModuleHandleA;
use d3d12_api::core::win32::system::threading::{CreateEventA, WaitForSingleObject};
use d3d12_api::core::win32::system::windows_programming::INFINITE;
use d3d12_api::core::win32::ui::windows_and_messaging::{AdjustWindowRectEx, CreateStructA, CreateWindowExA, CursorId, CW_USEDEFAULT, DefWindowProcA, DispatchMessageA, GetWindowLongPtrA, LoadCursorAById, PeekMessageA, PeekMessageRemoveType, RegisterClassExA, SetWindowLongPtrA, ShowWindow, ShowWindowCmd, TranslateMessage, WindowExStyle, WindowLongPtrIndex, WindowLongPtrUserData, WindowMessage, WindowMsg, WindowStyle, WndClassExA, WndClassStyles};
//use d3d12_api::extensions::win32::graphics::direct3d12::{ID3D12CommandQueueEx, ID3D12GraphicsCommandListEx};
//use d3d12_api::extensions::win32::graphics::dxgi::IDxgiInfoQueueEx;
use d3d12_api::helpers::FillRestWith;
use d3d12_api::hlsl_root_sig;

fn main() -> Result<(), Err> {
    let mut window = HelloTriangleWindow::new(640, 480)?;
    window.show();
    window.run_message_loop();
    Ok(())
}

#[derive(Debug)]
enum Err {
    HResult(HResult)
}

impl From<HResult> for Err {
    fn from(value: HResult) -> Self {
        #[cfg(debug_assertions)]
        panic!("{}", value);
        #[allow(unreachable_code)]
        Err::HResult(value)
    }
}

impl From<Win32Error> for Err {
    fn from(value: Win32Error) -> Self {
        panic!("{}", value)
    }
}

struct HelloTriangleWindow {
    window_handle: HWnd,
    resources: Resources,
}

// todo: destroying window mechanism
impl HelloTriangleWindow {
    fn new(width: u32, height: u32) -> Result<Pin<Box<Self>>, Err> {
        //
        let class_name = "hello-triangle-window\0";
        let instance = GetModuleHandleA(None)?;

        #[allow(invalid_value)]
            let window_class = WndClassExA {
            size: size_of::<WndClassExA>() as u32,
            style: WndClassStyles::HRedraw | WndClassStyles::VRedraw,
            wnd_proc: window_procedure::<Self>,
            instance,
            cursor: Some(LoadCursorAById(CursorId::Arrow)),
            class_name: class_name.as_pstr().unwrap(),
            ..unsafe { MaybeUninit::zeroed().assume_init() }
        };
        let atom = RegisterClassExA(&window_class).unwrap();
        let mut rect = Rect { left: 0, top: 0, right: width as i32, bottom: height as i32 };
        AdjustWindowRectEx(&mut rect, WindowStyle::OverlappedWindow, false, WindowExStyle::AppWindow).unwrap();

        // note: nightly: Box::new_uninit()
        let mut window = Box::new(MaybeUninit::<Self>::uninit());

        let window_handle = CreateWindowExA(
            WindowExStyle::AppWindow,
            Some(class_name),
            Some("hello triangle\0"),
            WindowStyle::OverlappedWindow,
            CW_USEDEFAULT, CW_USEDEFAULT,
            rect.width(), rect.height(),
            None,
            None,
            Some(instance),
            window.as_ptr() as _,
        )?;

        let (factory, _, device) = create_device()?;
        let resources = Resources::new(window_handle, width, height, &factory, &device)?;

        window.write(Self {
            window_handle,
            resources,
        });

        let window = unsafe {
            // note: nightly: window.assume_init()
            let window: Box<Self> = transmute(window);
            // note: nightly: Box::into_pin(window);
            let window: Pin<Box<Self>> = transmute(window);
            window
        };

        Ok(window)
    }

    fn show(&self) {
        ShowWindow(self.window_handle, ShowWindowCmd::Show);
    }

    fn run_message_loop(&mut self) {
        // note: keep self alive
        loop {
            if let Some(msg) = PeekMessageA(None, 0, 0, PeekMessageRemoveType::Remove) {
                match &msg {
                    WindowMsg::Msg(msg) => {
                        TranslateMessage(&msg);
                        DispatchMessageA(&msg);
                    }
                    WindowMsg::Quit(_) => break,
                }
            }
        }
    }
}

trait Window {
    fn on_paint(&mut self) -> Result<(), Err> { Err(Err::HResult(HResult::E_FAIL)) }
}

impl Window for HelloTriangleWindow {
    fn on_paint(&mut self) -> Result<(), Err> {
        self.resources.populate_command_list()?;
        self.resources.execute_command_list();
        self.resources.present()?;
        self.resources.wait_for_previous_frame()?;
        Ok(())
    }
}

extern "system" fn window_procedure<T: Window>(window_handle: HWnd, message: WindowMessage, w_param: WParam, l_param: LParam) -> LResult {
    fn window_procedure_core<T: Window>(window_handle: HWnd, message: WindowMessage, w_param: WParam, l_param: LParam) -> LResult {
        //
        if message == WindowMessage::Create && l_param.0 != 0 {
            let create_struct: &CreateStructA = unsafe { transmute(l_param) };
            SetWindowLongPtrA(window_handle, WindowLongPtrIndex::UserData, unsafe { transmute(create_struct.create_params) });
            return LResult::new(0);
        }

        let user_data = GetWindowLongPtrA::<WindowLongPtrUserData>(window_handle).unwrap().0;
        let window = if let Some(user_data) = user_data {
            unsafe {
                &mut *transmute::<NonZeroUsize, *mut T>(user_data)
            }
        } else {
            return DefWindowProcA(window_handle, message, w_param, l_param);
        };

        match message {
            WindowMessage::Paint => {
                if window.on_paint().is_ok() {
                    return LResult::new(0);
                }
            }
            _ => {}
        }

        DefWindowProcA(window_handle, message, w_param, l_param)
    }

    if let Ok(result) = catch_unwind(|| window_procedure_core::<T>(window_handle, message, w_param, l_param)) {
        result
    } else {
        DefWindowProcA(window_handle, message, w_param, l_param)
    }
}

struct Frame {
    command_queue: CommandQueue,
    swap_chain: SwapChain3,
    frame_index: u32,

    fence: Fence,
    fence_value: u64,
    fence_event: Handle,
}

impl Frame {
    //
    fn new(window_handle: HWnd, width: u32, height: u32, factory: &DxgiFactory2, device: &Device) -> Result<Self, Err> {
        //
        let command_queue: CommandQueue = device.CreateCommandQueue(&CommandQueueDesc::Direct)?;

        let swap_chain: SwapChain3 = factory.CreateSwapChainForHwnd(
            &command_queue,
            window_handle,
            &SwapChainDesc1 {
                buffer_count: Resources::FRAME_COUNT,
                width: width as u32,
                height: height as u32,
                format: Format::R8G8B8A8Unorm,
                buffer_usage: Usage::RenderTargetOutput,
                swap_effect: SwapEffect::FlipDiscard,
                alpha_mode: AlphaMode::Unspecified,
                sample_desc: SampleDesc::new(1, 0),
                stereo: false.into(),
                scaling: Scaling::Stretch,
                flags: DxgiSwapChainFlag::None,
            },
            None,
            None)?
            .QueryInterface()?;

        let frame_index = swap_chain.GetCurrentBackBufferIndex();
        let fence: Fence = device.CreateFence(0, FenceFlags::None)?;
        let fence_value = 0;
        let fence_event = CreateEventA(None, false, false, None)?;

        Ok(Self {
            command_queue,
            swap_chain,
            frame_index,
            fence,
            fence_value,
            fence_event,
        })
    }

    fn frame_index(&self) -> usize {
        self.frame_index as usize
    }

    fn execute_command_list(&self, command_list: &CommandList) {
        self.command_queue.execute_command_list(command_list)
    }

    fn present(&self) -> Result<(), Err> {
        self.swap_chain.Present(1, Present::None)?;
        Ok(())
    }

    fn wait_for_previous_frame(&mut self) -> Result<(), Err> {
        self.fence_value += 1;
        self.command_queue.Signal(&self.fence, self.fence_value)?;
        if self.fence.GetCompletedValue() != self.fence_value {
            self.fence.SetEventOnCompletion(self.fence_value, self.fence_event)?;
            WaitForSingleObject(self.fence_event, INFINITE);
        }
        self.frame_index = self.swap_chain.GetCurrentBackBufferIndex();
        Ok(())
    }
}

struct Resources {
    render_targets: [Resource; Resources::FRAME_COUNT as usize],
    rtv_heap: DescriptorHeap,
    rtv_descriptor_size: usize,

    frame: Frame,

    viewport: Viewport,
    scissor_rect: Rect,

    #[allow(dead_code)]
    vertex_buffer: Resource,
    vertex_buffer_view: VertexBufferView,

    root_signature: RootSignature,
    pipeline_state: PipelineState,

    command_allocator: CommandAllocator,
    command_list: GraphicsCommandList,
}

impl Resources {
    //
    const FRAME_COUNT: u32 = 2;

    fn new(window_handle: HWnd, width: u32, height: u32, factory: &DxgiFactory4, device: &Device) -> Result<Resources, Err> {
        //
        let frame = Frame::new(window_handle, width, height, factory, device)?;

        factory.MakeWindowAssociation(window_handle, DxgiMwa::NoAltEnter)?;

        let rtv_heap: DescriptorHeap =
            device.CreateDescriptorHeap(&DescriptorHeapDesc {
                num_descriptors: Resources::FRAME_COUNT,
                flags: DescriptorHeapFlags::None,
                ty: DescriptorHeapType::Rtv,
                node_mask: 0,
            })?;

        let rtv_descriptor_size =
            device.GetDescriptorHandleIncrementSize(DescriptorHeapType::Rtv) as usize;

        let rtv_handle = rtv_heap.GetCPUDescriptorHandleForHeapStart().ok_or_err()?;

        let render_targets: [Resource; Resources::FRAME_COUNT as usize] =
            try_array_init(|i: usize| -> Result<Resource, Err> {
                let render_target: Resource = frame.swap_chain.GetBuffer(i as u32)?;
                device.CreateRenderTargetView(
                    Some(&render_target), None,
                    rtv_handle + i * rtv_descriptor_size);
                Ok(render_target)
            })?;

        let viewport = Viewport::new(width as f32, height as f32);
        let scissor_rect = Rect::with_size(0, 0, width, height);

        {
            let mut feature_data = FeatureDataRootSignature { highest_version: RootSignatureVersion::_1_1 };
            device.CheckFeatureSupport(Feature::RootSignature, &mut feature_data)?;
        }
        let root_signature: RootSignature = device.CreateRootSignature(
            0,
            hlsl_root_sig!(RootFlags(AllowInputAssemblerInputLayout)))?;
        let pipeline_state = create_pipeline_state(device, &root_signature)?;

        let command_allocator: CommandAllocator = device.CreateCommandAllocator(CommandListType::Direct)?;
        let command_list: GraphicsCommandList =
            device.CreateCommandList(0, CommandListType::Direct, &command_allocator, None)?;
        command_list.Close()?;

        let aspect_ratio = width as f32 / height as f32;

        let (vertex_buffer, vertex_buffer_view) = create_vertex_buffer(device, aspect_ratio)?;

        let resources = Resources {
            render_targets,
            rtv_heap,
            rtv_descriptor_size,
            frame,
            viewport,
            scissor_rect,
            vertex_buffer,
            vertex_buffer_view,
            root_signature,
            pipeline_state,
            command_allocator,
            command_list,
        };

        Ok(resources)
    }

    fn rtv_handle(&self) -> CpuDescriptorHandle {
        self.rtv_heap.GetCPUDescriptorHandleForHeapStart().unwrap() + self.rtv_descriptor_size * self.frame.frame_index()
    }

    fn populate_command_list(&self) -> Result<(), Err> {
        //
        self.command_allocator.Reset()?;
        self.command_list.Reset(&self.command_allocator, Some(&self.pipeline_state))?;

        self.command_list.SetGraphicsRootSignature(Some(&self.root_signature));
        self.command_list.RSSetViewports(&[self.viewport]);
        self.command_list.RSSetScissorRects(&[self.scissor_rect]);

        self.command_list.resource_barrier_transition_all(
            &self.render_targets[self.frame.frame_index as usize],
            ResourceStates::Present, ResourceStates::RenderTarget);

        let rtv_handle = self.rtv_handle();
        self.command_list.om_set_render_target(Some(&rtv_handle), None);
        self.command_list.ClearRenderTargetView(rtv_handle, &[0.0, 0.2, 0.4, 1.0], None);
        self.command_list.IASetPrimitiveTopology(D3DPrimitiveTopology::TriangleList);
        self.command_list.IASetVertexBuffers(0, Some(&[self.vertex_buffer_view]));
        self.command_list.DrawInstanced(3, 1, 0, 0);

        self.command_list.resource_barrier_transition_all(
            &self.render_targets[self.frame.frame_index as usize],
            ResourceStates::RenderTarget, ResourceStates::Present);

        self.command_list.Close()?;

        Ok(())
    }

    fn execute_command_list(&self) {
        self.frame.execute_command_list(&self.command_list)
    }

    fn present(&self) -> Result<(), Err> {
        self.frame.present()
    }

    fn wait_for_previous_frame(&mut self) -> Result<(), Err> {
        self.frame.wait_for_previous_frame()
    }
}

fn create_device() -> Result<(Factory4, Adapter1, Device), Err> {
    #[cfg(debug_assertions)]
    {
        let mut debug: Option<Debug> = None;
        D3D12GetDebugInterface(Some(&mut debug))?;
        debug.ok_or_err()?.EnableDebugLayer();

        let info_queue = DXGIGetDebugInterface1::<InfoQueue>(0)?;
        info_queue.enable_break_on_error()?;
        info_queue.enable_break_on_corruption()?;
    }
    let flags = if cfg!(debug_assertions) { DXGI_CREATE_FACTORY_DEBUG } else { 0 };
    let factory = CreateDXGIFactory2::<Factory4>(flags)?;
    let adapter = get_hardware_adapter(&factory)?;

    let mut device: Option<Device> = None;
    D3D12CreateDevice(Some(adapter.as_unknown()), FeatureLevel::_11_0, Some(&mut device))?;

    Ok((factory, adapter, device.ok_or_err()?))
}

fn get_hardware_adapter(factory: &DxgiFactory1) -> Result<Adapter1, Err> {
    for i in 0.. {
        let adapter = factory.EnumAdapters1(i)?;
        let desc = adapter.GetDesc1()?;

        if desc.flags.contains(AdapterFlag::Software) {
            continue;
        }
        if D3D12CreateDevice::<Device>(Some(adapter.as_unknown()), FeatureLevel::_11_0, None).is_ok() {
            let desc = adapter.GetDesc1()?;
            println!("{:#?}", desc);
            return Ok(adapter);
        }
    }
    unreachable!()
}

fn create_pipeline_state(device: &Device, root_sig: &RootSignature) -> Result<PipelineState, Err> {
    let pipeline_state: PipelineState = device.CreateGraphicsPipelineState(#[allow(invalid_value)] &GraphicsPipelineStateDesc {
        root_signature: root_sig.as_root_signature().as_param(),
        vs: ShaderByteCode::new(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/hello-triangle-vs.dxil"))),
        ps: ShaderByteCode::new(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/hello-triangle-ps.dxil"))),
        blend_state: BlendDesc {
            alpha_to_coverage_enable: false.into(),
            independent_blend_enable: false.into(),
            render_target: [RenderTargetBlendDesc {
                blend_enable: false.into(),
                logic_op_enable: false.into(),
                src_blend: Blend::One,
                dest_blend: Blend::Zero,
                blend_op: BlendOp::Add,
                src_blend_alpha: Blend::One,
                dest_blend_alpha: Blend::One,
                blend_op_alpha: BlendOp::Add,
                logic_op: LogicOp::Noop,
                render_target_write_mask: ColorWriteEnable::All as u8,
            }].fill_rest_with(unsafe { MaybeUninit::zeroed().assume_init() }),
        },
        sample_mask: u32::MAX,
        rasterizer_state: RasterizerDesc {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::None,
            ..Default::default()
        },
        input_layout: InputLayoutDesc::new(&[
            InputElementDesc {
                semantic_name: "POSITION\0".as_pstr().unwrap(),
                semantic_index: 0,
                format: Format::R32G32B32Float,
                input_slot: 0,
                aligned_byte_offset: 0,
                input_slot_class: InputClassification::PerVertexData,
                instance_data_step_rate: 0,
            },
            InputElementDesc {
                semantic_name: "COLOR\0".as_pstr().unwrap(),
                semantic_index: 0,
                format: Format::R32G32B32A32Float,
                input_slot: 0,
                aligned_byte_offset: 12,
                input_slot_class: InputClassification::PerVertexData,
                instance_data_step_rate: 0,
            },
        ]),
        primitive_topology_type: PrimitiveTopologyType::Triangle,
        num_render_targets: 1,
        rtv_formats: [Format::R8G8B8A8Unorm].fill_rest_with(Format::Unknown),
        sample_desc: SampleDesc { count: 1, quality: 0 },
        ..unsafe { MaybeUninit::zeroed().assume_init() }
    })?;

    Ok(pipeline_state)
}

fn create_vertex_buffer(device: &Device, aspect_ratio: f32) -> Result<(Resource, VertexBufferView), Err> {
    struct Vertex {
        #[allow(dead_code)] position: [f32; 3],
        #[allow(dead_code)] color: [f32; 4],
    }

    let vertices = [
        Vertex { position: [0.0, 0.25 * aspect_ratio, 0.0], color: [1.0, 0.0, 0.0, 1.0] },
        Vertex { position: [0.25, -0.25 * aspect_ratio, 0.0], color: [0.0, 1.0, 0.0, 1.0] },
        Vertex { position: [-0.25, -0.25 * aspect_ratio, 0.0], color: [0.0, 0.0, 1.0, 1.0] }
    ];

    let mut vertex_buffer: Option<Resource> = None;
    device.CreateCommittedResource(
        &HeapProperties::UPLOAD,
        HeapFlags::None,
        &ResourceDesc::Buffer(size_of_val(&vertices) as u64),
        ResourceStates::GenericRead,
        None,
        Some(&mut vertex_buffer))?;
    let vertex_buffer = vertex_buffer.ok_or_err()?;

    let mut dest = None;
    vertex_buffer.Map(0, Some(&(0..0)), Some(&mut dest))?;
    let dest = dest.ok_or_err()?;
    unsafe {
        copy_nonoverlapping(vertices.as_ptr(), dest.as_ptr() as *mut Vertex, vertices.len());
    }
    vertex_buffer.Unmap(0, None);

    let vertex_buffer_view = VertexBufferView {
        buffer_location: vertex_buffer.GetGPUVirtualAddress().unwrap(),
        stride_in_bytes: size_of::<Vertex>() as u32,
        size_in_bytes: size_of_val(&vertices) as u32,
    };

    Ok((vertex_buffer, vertex_buffer_view))
}