use std::process::Command;
use std::fs;

fn main() {
    let glsl_dir = "src/shaders";
    let glsl_files = fs::read_dir(glsl_dir).unwrap();



    for file in glsl_files {
        let name = file.unwrap().path();
        let source = name.to_str().unwrap();

        //let shader_name = name.

        let out = source
            .split("/")
            .into_iter()
            .map(|x| {
                x.split(".")
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .iter()
                    .as_ref()
                [0]

            })
            .collect::<Vec<&str>>()
            .last()
            .copied()
            .unwrap()
            ;


        let mut t = Command::new("glslc");

        t.args(&[&source, "-o"])
            .arg(format!("target/debug/{}.spv", out))
            .status()
            .unwrap();

//        println!("cargo:warning={:?}", out);

    }

    println!("cargo::rerun-if-changed=src/shaders/simple_fragment.frag");
    println!("cargo::rerun-if-changed=src/shaders/simple_vertex.vert");
}