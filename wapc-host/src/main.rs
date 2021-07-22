use wapc::WapcHost;
use wasmtime_provider::WasmtimeEngineProvider;

fn host_callback(
    id: u64,
    bd: &str,
    ns: &str,
    op: &str,
    payload: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "Guest {} invoked '{}->{}:{}' with payload of {}",
        id,
        bd,
        ns,
        op,
        ::std::str::from_utf8(payload).unwrap()
    );
    Ok(vec![])
}

pub fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let module_bytes = std::fs::read(&std::env::args().skip(1).next().unwrap()).unwrap();
    let engine = WasmtimeEngineProvider::new(&module_bytes, None);
    let host = WapcHost::new(Box::new(engine), host_callback)?;
    let func = std::env::args().skip(2).next().unwrap();

    let _res = host.call(&func, b"this is a test")?;
    Ok(())
}
