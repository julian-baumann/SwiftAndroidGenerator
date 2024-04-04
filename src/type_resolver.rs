use std::str::Chars;

#[derive(Debug)]
pub enum JavaType {
    Int,
    Long,
    Float,
    Double,
    Byte,
    Char,
    Short,
    Boolean,
    Array(Box<JavaType>),
    Reference(String),
    Void
}

#[derive(Debug)]
pub struct MethodSignature {
    pub parameters: Vec<JavaType>,
    pub return_type: JavaType,
}

pub enum GetTypeResponse {
    Some(JavaType),
    None,
    EndOfParsing
}

fn get_type(character: char, chars: &mut Chars) -> GetTypeResponse {
    return match character {
        'I' => GetTypeResponse::Some(JavaType::Int),
        'J' => GetTypeResponse::Some(JavaType::Long),
        'F' => GetTypeResponse::Some(JavaType::Float),
        'D' => GetTypeResponse::Some(JavaType::Double),
        'B' => GetTypeResponse::Some(JavaType::Byte),
        'C' => GetTypeResponse::Some(JavaType::Char),
        'S' => GetTypeResponse::Some(JavaType::Short),
        'Z' => GetTypeResponse::Some(JavaType::Boolean),
        'V' => GetTypeResponse::Some(JavaType::Void),
        '[' => {
            let java_type = get_type(chars.next().expect("Expected type after ["), chars);

            if let GetTypeResponse::Some(java_type) = java_type {
                return GetTypeResponse::Some(JavaType::Array(Box::new(java_type)));
            }

            return GetTypeResponse::None;
        }
        'L' => {
            let mut obj_name = String::new();
            while let Some(c) = chars.next() {
                if c == ';' {
                    break;
                }
                obj_name.push(c);
            }
            GetTypeResponse::Some(JavaType::Reference(obj_name))
        },
        '(' => GetTypeResponse::None,
        ')' => GetTypeResponse::EndOfParsing,
        _ => GetTypeResponse::None,
    }
}

pub fn parse_signature(signature: &str) -> MethodSignature {
    let mut chars = signature.chars();
    let mut parameters = Vec::new();

    // Parse parameters
    while let Some(current_char) = chars.next() {
        let parameter = get_type(current_char, &mut chars);

        match parameter {
            GetTypeResponse::Some(java_parameter) => {
                parameters.push(java_parameter);
            }
            GetTypeResponse::None => {}
            GetTypeResponse::EndOfParsing => {
                break;
            }
        }
    }

    // Parse return type
    let return_type_char = chars.next().expect("No return type specified");
    let return_type = get_type(return_type_char, &mut chars);
    let return_type = match return_type {
        GetTypeResponse::Some(java_type) => java_type,
        _ => panic!("Failed to parse return type")
    };

    MethodSignature {
        parameters,
        return_type,
    }
}
