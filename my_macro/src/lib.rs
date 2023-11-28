extern crate proc_macro;

use proc_macro::*;

// Example:
// [add_containers_from_files("src/scene/*.rs")]
// struct Scene {
// }

#[proc_macro_attribute]
pub fn add_containers_from_files(args: TokenStream, input: TokenStream) -> TokenStream {
    // get glob pattern from args remove quotes
    let args_str = args.to_string();
    let args_str = args_str
        .split("\"")
        .nth(1)
        .expect("Failed to get glob pattern");
    let glob_pattern = std::path::Path::new(args_str);

    // get struct name from input
    let struct_name = input.to_string().split("struct ").collect::<Vec<&str>>()[1]
        .split(" {")
        .collect::<Vec<&str>>()[0]
        .to_string();

    // get all files from glob pattern
    // and for each file get the namespace and format them
    let names = glob::glob(
        glob_pattern
            .to_str()
            .expect("Failed to convert path to str"),
    )
    .expect("Failed to read glob pattern")
    .map(|x| x.expect("Failed to read file"))
    .map(|x| {
        // Convert path to name of namespace of file + file without extension
        // Example: src/scene/sphere.rs -> sphere::Sphere
        let path = x.to_str().expect("Failed to convert path to str");
        let path = path.split("/").collect::<Vec<&str>>();
        let namespace = path.last().expect("Failed to get namespace");
        let namespace = namespace.split(".").collect::<Vec<&str>>()[0];

        namespace.to_string()
    })
    .collect::<Vec<String>>();

    if names.len() == 0 {
        panic!(
            "No files found for glob pattern: {}",
            glob_pattern.display()
        );
    }

    let containers_string = names
        .iter()
        .map(|x| {
            format!(
                "{}s: Vec<{}::{}>,",
                x,
                x,
                x.chars()
                    .nth(0)
                    .expect("Failed to get first char")
                    .to_ascii_uppercase()
                    .to_string()
                    + &x[1..]
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    let containers_new_string = names
        .iter()
        .map(|x| format!("{}s: Vec::new(),", x))
        .collect::<Vec<String>>()
        .join("\n");
    let namespaces_string = names
        .iter()
        .map(|x| format!("pub mod {};", x))
        .collect::<Vec<String>>()
        .join("\n");
    let add_functions = names
        .iter()
        .map(|x| {
            format!(
                "pub fn add_{}(&mut self, {}: {}::{}) {{
                    self.{}s.push({});
                }}",
                x,
                x,
                x,
                x.chars()
                    .nth(0)
                    .expect("Failed to get first char")
                    .to_ascii_uppercase()
                    .to_string()
                    + &x[1..],
                x,
                x
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    let intersect_functions = names
        .iter()
        .map(|x| {
            format!(
                "fn intersect_{}(&self, ray: &ray::Ray) -> Option<(f64, crate::vec3::UnitVec3)> {{
                    self.{}s.iter().map(|obj| obj.intersect(ray))
                        .filter(|x| x.is_some())
                        .map(|x| x.unwrap())
                        .nth(0)
                }}",
                x, x
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    let intersect_function_calls = names
        .iter()
        .map(|x| {
            format!(
                "let {}s_ret = self.intersect_{}(ray); if {}s_ret.is_some() {{ return {}s_ret }}",
                x, x, x, x
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", containers_string);
    println!("{}", containers_new_string);
    println!("{}", namespaces_string);
    format!(
        "
        {namespaces_string}

        pub struct {struct_name} {{
            {containers_string}
        }}

        impl Scene {{
            pub fn new() -> Self {{
                Self {{
                    {containers_new_string}
                }}
            }}

            {add_functions}

            {intersect_functions}

            pub fn intersect(&self, ray: &ray::Ray) -> Option<(f64, crate::vec3::UnitVec3)> {{
                {intersect_function_calls}

                None
            }}
        }}"
    )
    .parse::<TokenStream>()
    .expect("Failed to parse TokenStream")
}
