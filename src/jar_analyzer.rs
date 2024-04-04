use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use jars::JarOptionBuilder;
use jbcrs::basic::AccessFlags;
use jbcrs::basic::Item::UTF8;
use crate::type_resolver::{JavaType, parse_signature};

pub struct JavaMethod {
    pub name: String,
    pub access_flags: AccessFlags,
    pub parameters: Vec<JavaType>,
    pub return_type: JavaType,
}

impl Display for JavaMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}({:?}) -> {:?}", self.access_flags, self.name, self.parameters, self.return_type)
    }
}

pub struct JavaClass {
    pub full_name: String,
    pub name: String,
    pub methods: Vec<JavaMethod>
}

pub struct JarAnalyzer {
    class_files: HashMap<String, Vec<u8>>
}

impl JarAnalyzer {
    pub fn new(jar_path: String) -> Self {
        let jar = jars::jar(jar_path, JarOptionBuilder::builder().ext("class").build())
            .expect("Failed to open jar file.");

        Self {
            class_files: jar.files
        }
    }

    pub fn get_classes(&self) -> Vec<JavaClass> {
        let mut classes = vec![];

        for (file_path, content) in &self.class_files {
            let parse_result = jbcrs::basic::parse(&content);

            let Ok((pool, class)) = parse_result else {
                continue;
            };

            let mut methods = vec![];

            for method in class.methods {
                if let UTF8(signature) = pool.get(method.desc).unwrap() {
                    let parsed_signature = parse_signature(signature);

                    let method = JavaMethod {
                        name: pool.get_utf8(method.name).expect("Failed to get class name"),
                        access_flags: method.access_flags,
                        parameters: parsed_signature.parameters,
                        return_type: parsed_signature.return_type,
                    };

                    println!("{}", method);

                    methods.push(method)
                }
            }

            classes.push(JavaClass {
                full_name: file_path.clone(),
                name: file_path.clone(),
                methods,
            })
        }

        return classes;
    }
}
