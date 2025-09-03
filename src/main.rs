use wasmtime::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== WASM Runner ===");

    // 1. Utwórz engine i wczytaj moduł WASM
    let engine = Engine::default();
    let module = Module::from_file(&engine, "src/simple.wat")?;
    
    // 2. Utwórz store (kontekst wykonania)
    let mut store = Store::new(&engine, ());

    // 3. Utwórz instancję modułu (bez importów - to standalone WASM)
    let instance = Instance::new(&mut store, &module, &[])?;

    println!("✅ WASM module loaded successfully!");

    // 4. Pobierz funkcję 'add'
    let add_func = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "add")?;

    // 5. Wywołaj funkcję!
    println!("\n=== Testowanie funkcji add ===");
    
    let test_cases = [
        (5, 3),
        (10, 20),
        (-5, 8),
        (0, 0),
        (100, -50),
    ];

    for (a, b) in test_cases {
        let result = add_func.call(&mut store, (a, b))?;
        println!("{} + {} = {}", a, b, result);
    }

    // 6. Sprawdź inne eksportowane funkcje (opcjonalne)
    println!("\n=== Dostępne eksporty ===");
    for export in instance.exports(&mut store) {
        match export.clone().into_extern() {
            Extern::Func(_) => println!("📦 Funkcja: {}", export.name()),
            Extern::Memory(_) => println!("🧠 Memory: {}", export.name()),
            Extern::Table(_) => println!("📋 Table: {}", export.name()),
            Extern::Global(_) => println!("🌍 Global: {}", export.name()),
            Extern::SharedMemory(_) => println!("🔗 Shared Memory: {}", export.name()),
            Extern::Tag(_) => println!("🏷️ Tag: {}", export.name()),
        }
    }

    // 7. Sprawdź memory (opcjonalne)
    if let Some(memory) = instance.get_memory(&mut store, "memory") {
        let size = memory.size(&store);
        println!("\n🧠 WASM Memory: {} stron ({} bytes)", size, size * 65536);
    }

    println!("\n✅ Wszystko działa!");
    Ok(())
}