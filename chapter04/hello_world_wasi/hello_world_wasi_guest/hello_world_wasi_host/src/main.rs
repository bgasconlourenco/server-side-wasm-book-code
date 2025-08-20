//host implementation
wasmtime::component::bindgen!({
    path: "../wit",
    world: "example",
}); 

struct State{
    wasi: wasmtime_wasi::WasiCtx,
    table: wasmtime_wasi::ResourceTable,
}
//give access to the WASI context
impl wasmtime_wasi::WasiView for State {
    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.wasi
    }
}
//give access to the resource table
impl wasmtime_wasi::IoView for State {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
 
}

fn main() {
    let mut config = wasmtime::Config::default();
    config.wasm_component_model(true); //hold configuration for wasmtime engine 
    //instance to load and run js component
    let engine = wasmtime::Engine::new(&config).unwrap(); 
    //associate the state with the engine
    let mut linker = wasmtime::component::Linker::<State>::new(&engine); 
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap(); 
    //tie the context's output stream to the host's stdout
    let wasi = wasmtime_wasi::WasiCtxBuilder::new() 
        .inherit_stdout()
        .build();
    //create store object and load greet component from greet.wasm 
    let mut store = wasmtime::Store::new(&engine, State { wasi, table: wasmtime_wasi::ResourceTable::new() });
    let component = wasmtime::component::Component::from_file(&engine, "../hello_world_wasi_guest/greet.wasm").unwrap();
    //instantiante component and call greet function
    let app = Example::instantiate(&mut store, &component, &linker).unwrap();
    app.call_greet(&mut store, "World").unwrap();

}