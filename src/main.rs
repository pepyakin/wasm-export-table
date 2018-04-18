extern crate parity_wasm;

use parity_wasm::elements;
use std::env;

fn usage() {
    println!("wasm-export-table input output")
}

fn module_contains_table(module: &elements::Module) -> bool {
    // First, look into a table section for the declared table.
    if let Some(table_section) = module.table_section() {
        if table_section.entries().len() > 0 {
            return true;
        }
    }

    // If there is no declared tables, we can still import one.
    if let Some(import_section) = module.import_section() {
        let table_import_found = import_section.entries().iter().any(|ref entry| {
            match *entry.external() {
                elements::External::Table(_) => true,
                _ => false,
            }
        });
        if table_import_found {
            return true;
        }
    }

    // There is no tables in the module.
    false
}

fn add_export_table(module: &mut elements::Module) {
    for section in module.sections_mut() {
        match *section {
            elements::Section::Export(ref mut export_section) => {
                // Search for any table exports in the export section.
                let table_export_found = export_section.entries().iter().any(|ref entry| {
                    if let elements::Internal::Table(_) = *entry.internal() {
                        true
                    } else {
                        false
                    }
                });

                if !table_export_found {
                    // TODO: Specify table name via args
                    export_section
                        .entries_mut()
                        .push(elements::ExportEntry::new(
                            "table".to_string(),
                            elements::Internal::Table(0),
                        ))
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        usage();
        return;
    }

    let mut module = elements::deserialize_file(&args[1]).expect("Failed to deserialize the file");

    if module_contains_table(&module) {
        add_export_table(&mut module);
    }

    // TODO: What if there is no export section?

    elements::serialize_to_file(&args[2], module).expect("Failed to serialize the result");
}
