use std::fs;
use std::fs::File;
use std::io::Write;
use d3d12_api::aliases::win32::graphics::direct3d::dxc::{Buffer, Compiler3, OperationResult};
use d3d12_api::core::win32::foundation::{HResult, OkOrErr};

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/hello-triangle.hlsl");
    let content = fs::read_to_string(path).unwrap();
    let compiler = Compiler3::new().unwrap();
    create_dxil(
        &compiler,
        content.as_str(),
        &["-E VSMain", "-T vs_6_5", "-O0", "-Zi"],
        "/shaders/hello-triangle-vs.dxil",
    ).unwrap();
    create_dxil(
        &compiler,
        content.as_str(),
        &["-E PSMain", "-T ps_6_5", "-O0", "-Zi"],
        "/shaders/hello-triangle-ps.dxil",
    ).unwrap();
}

fn create_dxil(compiler: &Compiler3, content: &str, options: &[&str], output_path: &str) -> Result<(), HResult> {
    let result: OperationResult = compiler.Compile(
        &Buffer::new(content),
        Some(options),
        None,
    )?;
    let status = result.GetStatus()?;
    if status.is_err() {
        println!("{}", result.get_error_buffer()?);
        return Err(HResult::E_FAIL);
    }
    let blob = result.get_result()?;

    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), output_path);
    let mut file = File::create(path).unwrap();
    file.write_all(blob.as_ref()).ok().ok_or_err()?;
    Ok(())
}