#![allow(unused_imports)]

use crate::core::win32::graphics::direct3d::dxc::*;

pub type Cp = DxcCp;
pub type ShaderHash = DxcShaderHash;
pub type Blob = DxcBlob;
pub trait IBlob: IDxcBlob {}
impl<T: IDxcBlob> IBlob for T {}
pub type BlobEncoding = DxcBlobEncoding;
pub trait IBlobEncoding: IDxcBlobEncoding {}
impl<T: IDxcBlobEncoding> IBlobEncoding for T {}
pub type BlobUtf16 = DxcBlobUtf16;
pub trait IBlobUtf16: IDxcBlobUtf16 {}
impl<T: IDxcBlobUtf16> IBlobUtf16 for T {}
pub type BlobUtf8 = DxcBlobUtf8;
pub trait IBlobUtf8: IDxcBlobUtf8 {}
impl<T: IDxcBlobUtf8> IBlobUtf8 for T {}
pub type IncludeHandler = DxcIncludeHandler;
pub trait IIncludeHandler: IDxcIncludeHandler {}
impl<T: IDxcIncludeHandler> IIncludeHandler for T {}
pub type Buffer<'a> = DxcBuffer<'a>;
pub type Define<'a> = DxcDefine<'a>;
pub type CompilerArgs = DxcCompilerArgs;
pub trait ICompilerArgs: IDxcCompilerArgs {}
impl<T: IDxcCompilerArgs> ICompilerArgs for T {}
pub type Library = DxcLibrary;
pub trait ILibrary: IDxcLibrary {}
impl<T: IDxcLibrary> ILibrary for T {}
pub type OperationResult = DxcOperationResult;
pub trait IOperationResult: IDxcOperationResult {}
impl<T: IDxcOperationResult> IOperationResult for T {}
pub type Compiler = DxcCompiler;
pub trait ICompiler: IDxcCompiler {}
impl<T: IDxcCompiler> ICompiler for T {}
pub type Compiler2 = DxcCompiler2;
pub trait ICompiler2: IDxcCompiler2 {}
impl<T: IDxcCompiler2> ICompiler2 for T {}
pub type Linker = DxcLinker;
pub trait ILinker: IDxcLinker {}
impl<T: IDxcLinker> ILinker for T {}
pub type Utils = DxcUtils;
pub trait IUtils: IDxcUtils {}
impl<T: IDxcUtils> IUtils for T {}
pub type OutKind = DxcOutKind;
pub type Result = DxcResult;
pub trait IResult: IDxcResult {}
impl<T: IDxcResult> IResult for T {}
pub type ExtraOutputs = DxcExtraOutputs;
pub trait IExtraOutputs: IDxcExtraOutputs {}
impl<T: IDxcExtraOutputs> IExtraOutputs for T {}
pub type Compiler3 = DxcCompiler3;
pub trait ICompiler3: IDxcCompiler3 {}
impl<T: IDxcCompiler3> ICompiler3 for T {}
pub type Validator = DxcValidator;
pub trait IValidator: IDxcValidator {}
impl<T: IDxcValidator> IValidator for T {}
pub type Validator2 = DxcValidator2;
pub trait IValidator2: IDxcValidator2 {}
impl<T: IDxcValidator2> IValidator2 for T {}
pub type ContainerBuilder = DxcContainerBuilder;
pub trait IContainerBuilder: IDxcContainerBuilder {}
impl<T: IDxcContainerBuilder> IContainerBuilder for T {}
pub type Assembler = DxcAssembler;
pub trait IAssembler: IDxcAssembler {}
impl<T: IDxcAssembler> IAssembler for T {}
pub type OptimizerPass = DxcOptimizerPass;
pub trait IOptimizerPass: IDxcOptimizerPass {}
impl<T: IDxcOptimizerPass> IOptimizerPass for T {}
pub type Optimizer = DxcOptimizer;
pub trait IOptimizer: IDxcOptimizer {}
impl<T: IDxcOptimizer> IOptimizer for T {}
pub type VersionInfo = DxcVersionInfo;
pub trait IVersionInfo: IDxcVersionInfo {}
impl<T: IDxcVersionInfo> IVersionInfo for T {}
pub type VersionInfo2 = DxcVersionInfo2;
pub trait IVersionInfo2: IDxcVersionInfo2 {}
impl<T: IDxcVersionInfo2> IVersionInfo2 for T {}
pub type VersionInfo3 = DxcVersionInfo3;
pub trait IVersionInfo3: IDxcVersionInfo3 {}
impl<T: IDxcVersionInfo3> IVersionInfo3 for T {}
pub type ArgPair<'a> = DxcArgPair<'a>;
pub type PdbUtils = DxcPdbUtils;
pub trait IPdbUtils: IDxcPdbUtils {}
impl<T: IDxcPdbUtils> IPdbUtils for T {}
