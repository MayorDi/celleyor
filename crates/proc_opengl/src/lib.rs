use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree};

extern crate proc_macro;

#[proc_macro]
pub fn init_program_shader(item: TokenStream) -> TokenStream {
    let mut program_name_and_paths_shaders = HashMap::<String, (String, String)>::new();

    let mut is_new_program = false;
    let mut name_program = String::new();
    for token in item.into_iter() {
        if let TokenTree::Ident(name) = token {
            is_new_program = true;
            name_program = name.to_string();
            continue;
        }

        if is_new_program {
            if let TokenTree::Group(group) = token {
                let paths = get_paths(group.stream());
                program_name_and_paths_shaders.insert(name_program.clone(), paths);
                is_new_program = false;
                name_program.clear();
            }
        }
    }

    let mut structure = "pub struct Programs {\n".to_owned();
    let mut impl_for_struct = "impl Programs {\n".to_owned();
    let mut init_fn = "pub fn init() -> Self {\n Self {\n".to_owned();
    for program in program_name_and_paths_shaders.iter() {
        structure.push_str(format!("pub {}: Program<Shader>,\n", program.0.as_str()).as_str());
        init_fn.push_str(
            format!("{}: Self::{}(),\n", program.0.as_str(), program.0.as_str()).as_str(),
        );
        impl_for_struct.push_str(create_fn_build(program.0, program.1).as_str());
    }
    structure.push_str("\n}\n");
    init_fn.push_str("\n}\n}\n");
    impl_for_struct.push_str(init_fn.as_str());
    impl_for_struct.push_str("\n}\n");

    let mut res = String::new();
    res.push_str(structure.as_str());
    res.push_str(impl_for_struct.as_str());

    res.as_str().parse().unwrap()
}

fn create_fn_build(name_program: &String, paths: &(String, String)) -> String {
    format!(
        r#"
        fn {name_program}() -> Program<Shader> {{
        let vs = Shader::new(
            gl::VERTEX_SHADER,
            load_bytes_from_file({}).unwrap(),
        );
        let fs = Shader::new(
            gl::FRAGMENT_SHADER,
            load_bytes_from_file({}).unwrap(),
        );

        let mut program = Program::new();
        program.push_shader(vs);
        program.push_shader(fs);
        program.build().unwrap();

        program
    }}
    "#,
        paths.0.as_str(),
        paths.1.as_str()
    )
}

fn get_paths(tokens: TokenStream) -> (String, String) {
    let mut vs = String::new();
    let mut fs = String::new();
    let mut is_vs @ mut is_fs = false;
    for token in tokens.into_iter() {
        if let TokenTree::Ident(ident) = &token {
            if ident.to_string() == "vs".to_owned() {
                is_vs = true;
                is_fs = false;
            }

            if ident.to_string() == "fs".to_owned() {
                is_vs = false;
                is_fs = true;
            }
        }

        if let TokenTree::Literal(path) = &token {
            if is_vs {
                vs = path.to_string();
            } else if is_fs {
                fs = path.to_string();
            }
        }
    }

    (vs, fs)
}
