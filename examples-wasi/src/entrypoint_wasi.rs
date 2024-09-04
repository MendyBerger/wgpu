wit_bindgen::generate!({
    path: "wit",
    world: "example:example/example",
});

struct ExampleDesc {
    name: &'static str,
    function: fn(),
    #[allow(dead_code)] // isn't used on native
    webgl: bool,
    #[allow(dead_code)] // isn't used on native
    webgpu: bool,
}

const EXAMPLES: &[ExampleDesc] = &[
    // ExampleDesc {
    //     name: "boids",
    //     function: crate::boids::main,
    //     webgl: false, // No compute
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "bunnymark",
    //     function: crate::bunnymark::main,
    //     webgl: true,
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "conservative_raster",
    //     function: crate::conservative_raster::main,
    //     webgl: false,  // No conservative raster
    //     webgpu: false, // No conservative raster
    // },
    // ExampleDesc {
    //     name: "cube",
    //     function: crate::cube::main,
    //     webgl: true,
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "hello",
    //     function: crate::hello::main,
    //     webgl: false, // No canvas for WebGL
    //     webgpu: true,
    // },
    ExampleDesc {
        name: "hello_compute",
        function: crate::hello_compute::main,
        webgl: false, // No compute
        webgpu: true,
    },
    // ExampleDesc {
    //     name: "hello_synchronization",
    //     function: crate::hello_synchronization::main,
    //     webgl: false, // No canvas for WebGL
    //     webgpu: true,
    // },
    ExampleDesc {
        name: "hello_triangle",
        function: crate::hello_triangle::main,
        webgl: true,
        webgpu: true,
    },
    // ExampleDesc {
    //     name: "hello_windows",
    //     function: crate::hello_windows::main,
    //     webgl: false,  // Native only example
    //     webgpu: false, // Native only example
    // },
    // ExampleDesc {
    //     name: "hello_workgroups",
    //     function: crate::hello_workgroups::main,
    //     webgl: false,
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "mipmap",
    //     function: crate::mipmap::main,
    //     webgl: true,
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "msaa_line",
    //     function: crate::msaa_line::main,
    //     webgl: true,
    //     webgpu: true,
    // },
    ExampleDesc {
        name: "render_to_texture",
        function: crate::render_to_texture::main,
        webgl: false, // No canvas for WebGL
        webgpu: true,
    },
    ExampleDesc {
        name: "repeated_compute",
        function: crate::repeated_compute::main,
        webgl: false, // No compute
        webgpu: true,
    },
    ExampleDesc {
        name: "shadow",
        function: crate::shadow::main,
        webgl: true,
        webgpu: true,
    },
    // ExampleDesc {
    //     name: "skybox",
    //     function: crate::skybox::main,
    //     webgl: true,
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "srgb_blend",
    //     function: crate::srgb_blend::main,
    //     webgl: true,
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "stencil_triangles",
    //     function: crate::stencil_triangles::main,
    //     webgl: true,
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "storage_texture",
    //     function: crate::storage_texture::main,
    //     webgl: false, // No storage textures
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "texture_arrays",
    //     function: crate::texture_arrays::main,
    //     webgl: false,  // No texture arrays
    //     webgpu: false, // No texture arrays
    // },
    // ExampleDesc {
    //     name: "timestamp_queries",
    //     function: crate::timestamp_queries::main,
    //     webgl: false,  // No canvas for WebGL
    //     webgpu: false, // No timestamp queries
    // },
    // ExampleDesc {
    //     name: "uniform_values",
    //     function: crate::uniform_values::main,
    //     webgl: false, // No compute
    //     webgpu: true,
    // },
    // ExampleDesc {
    //     name: "water",
    //     function: crate::water::main,
    //     webgl: false, // No RODS
    //     webgpu: true,
    // },
];

// fn get_example_name() -> Option<String> {
//     // cfg_if::cfg_if! {
//     //     if #[cfg(target_arch = "wasm32")] {
//     //         let query_string = web_sys::window()?.location().search().ok()?;

//     //         crate::framework::parse_url_query_string(&query_string, "example").map(String::from)
//     //     } else {
//             std::env::args().nth(1)
//     //     }
//     // }
// }

// #[cfg(target_arch = "wasm32")]
// fn print_examples() {
//     // Get the document, header, and body elements.
//     let document = web_sys::window().unwrap().document().unwrap();

//     for backend in ["webgl2", "webgpu"] {
//         let ul = document
//             .get_element_by_id(&format!("{backend}-list"))
//             .unwrap();

//         for example in EXAMPLES {
//             if backend == "webgl2" && !example.webgl {
//                 continue;
//             }
//             if backend == "webgpu" && !example.webgpu {
//                 continue;
//             }

//             let link = document.create_element("a").unwrap();
//             link.set_text_content(Some(example.name));
//             link.set_attribute(
//                 "href",
//                 &format!("?backend={backend}&example={}", example.name),
//             )
//             .unwrap();
//             link.set_class_name("example-link");

//             let item = document.create_element("div").unwrap();
//             item.append_child(&link).unwrap();
//             item.set_class_name("example-item");
//             ul.append_child(&item).unwrap();
//         }
//     }
// }

// #[cfg(target_arch = "wasm32")]
// fn print_unknown_example(_result: Option<String>) {}

// #[cfg(not(target_arch = "wasm32"))]
fn print_unknown_example(result: Option<String>) {
    if let Some(example) = result {
        println!("Unknown example: {}", example);
    } else {
        println!("Please specify an example as the first argument!");
    }

    println!("\nAvailable Examples:");
    for examples in EXAMPLES {
        println!("\t{}", examples.name);
    }
}

struct Example;

impl Guest for Example {
    fn start(example: String) {
        std::panic::set_hook(Box::new(|info: _| {
            print(&format!("panic message: {info}"));
        }));

        print(&example);

        let Some(found) = EXAMPLES.iter().find(|e| e.name == example) else {
            print_unknown_example(Some(example));
            return;
        };

        (found.function)();
    }
}

export!(Example);

use log::LevelFilter;
use log::{Metadata, Record};

pub(crate) struct WasiLogger;

impl WasiLogger {
    pub fn init() {
        static LOGGER: WasiLogger = WasiLogger;
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);
    }
}

impl log::Log for WasiLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        print(&format!(
            "wasi-logger: {} - {}",
            record.level(),
            record.args()
        ));
    }

    fn flush(&self) {}
}
