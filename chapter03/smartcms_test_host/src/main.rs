wasmtime::component::bindgen!({ //glue code generation
    path: "./smart_cms.wit",
    world: "app",
});

struct KeyValue { //host trait implementation
    mem: std::collections::HashMap<String, String>,
}

impl component::smartcms::kvstore::Host for KeyValue {
    fn get(&mut self, key: String) -> Option<String> {
        self.mem.get(&key).cloned()
    }

    fn set(&mut self, key: String, value: String) {
        self.mem.insert(key, value);
    }
}

struct State{ //state to hold the host trait implementation
    key_value: KeyValue,
}

fn main() { //main function to instantiate the component and run it
    let mut config = wasmtime::Config::default(); // create a new configuration
    config.wasm_component_model(true); //enable component model

    let engine = wasmtime::Engine::new(&config).unwrap(); // create a new engine with the configuration

    let mut store = wasmtime::Store::new(&engine, State { key_value: KeyValue { mem: std::collections::HashMap::new(),},}); // create a new store with the state

    let component = wasmtime::component::Component::from_file(&engine, "guest.wasm").unwrap(); // load the component from the file

    let mut linker = wasmtime::component::Linker::new(&engine); // create a new linker
    component::smartcms::kvstore::add_to_linker(&mut linker, |state: &mut State| &mut state.key_value).unwrap(); // add the host trait implementation to the linker

    let app = App::instantiate(&mut store, &component, &linker).unwrap(); // instantiate the component with the store and linker

    println!("{:?}", app.call_run(&mut store).unwrap()); // call the run function of the component    
}
