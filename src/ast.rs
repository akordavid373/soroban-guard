use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAst {
    pub name: String,
    pub functions: Vec<Function>,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub imports: Vec<String>,
    pub constants: Vec<Constant>,
    pub type_aliases: Vec<TypeAlias>,
    pub traits: Vec<Trait>,
    pub impl_blocks: Vec<ImplBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub is_mut: bool,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<String>,
    pub line_start: usize,
    pub line_end: usize,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<String>,
    pub where_clause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub visibility: Visibility,
    pub line_start: usize,
    pub line_end: usize,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
    pub visibility: Visibility,
    pub line_start: usize,
    pub line_end: usize,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub is_mut: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Type>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub const_type: Type,
    pub value: String,
    pub visibility: Visibility,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub target_type: Type,
    pub visibility: Visibility,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<Function>,
    pub line_start: usize,
    pub line_end: usize,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplBlock {
    pub target_type: Type,
    pub trait_name: Option<String>,
    pub methods: Vec<Function>,
    pub line_start: usize,
    pub line_end: usize,
    pub generics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<String>,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Simple(String),
    Generic {
        name: String,
        args: Vec<Type>,
    },
    Tuple(Vec<Type>),
    Array {
        element_type: Box<Type>,
        size: Option<String>,
    },
    Slice(Box<Type>),
    Reference {
        target: Box<Type>,
        is_mut: bool,
    },
    RawPointer {
        target: Box<Type>,
        is_mut: bool,
    },
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
        is_unsafe: bool,
    },
    Never,
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Crate,
    Restricted,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Simple(name) => write!(f, "{}", name),
            Type::Generic { name, args } => {
                write!(f, "{}<", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ">")
            }
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, typ) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", typ)?;
                }
                write!(f, ")")
            }
            Type::Array { element_type, size } => {
                write!(f, "[{}]", element_type)?;
                if let Some(size) = size {
                    write!(f, "; {}", size)?;
                }
                Ok(())
            }
            Type::Slice(element_type) => write!(f, "[{}]", element_type),
            Type::Reference { target, is_mut } => {
                write!(f, "&")?;
                if *is_mut {
                    write!(f, "mut ")?;
                }
                write!(f, "{}", target)
            }
            Type::RawPointer { target, is_mut } => {
                write!(f, "*")?;
                if *is_mut {
                    write!(f, "mut ")?;
                } else {
                    write!(f, "const ")?;
                }
                write!(f, "{}", target)
            }
            Type::Function { parameters, return_type, is_unsafe } => {
                if *is_unsafe {
                    write!(f, "unsafe ")?;
                }
                write!(f, "fn(")?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::Never => write!(f, "!"),
            Type::Unit => write!(f, "()"),
        }
    }
}

pub struct AstParser;

impl AstParser {
    pub fn parse_file(file_path: &str) -> anyhow::Result<ContractAst> {
        let source_code = std::fs::read_to_string(file_path)?;
        Self::parse_source(&source_code, file_path)
    }

    pub fn parse_source(source_code: &str, file_path: &str) -> anyhow::Result<ContractAst> {
        let mut contract = ContractAst {
            name: Self::extract_contract_name(source_code, file_path)?,
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            imports: Vec::new(),
            constants: Vec::new(),
            type_aliases: Vec::new(),
            traits: Vec::new(),
            impl_blocks: Vec::new(),
        };

        // Parse imports
        contract.imports = Self::extract_imports(source_code);

        // Parse functions
        contract.functions = Self::extract_functions(source_code)?;

        // Parse structs
        contract.structs = Self::extract_structs(source_code)?;

        // Parse enums
        contract.enums = Self::extract_enums(source_code)?;

        // Parse constants
        contract.constants = Self::extract_constants(source_code)?;

        // Parse type aliases
        contract.type_aliases = Self::extract_type_aliases(source_code)?;

        // Parse traits
        contract.traits = Self::extract_traits(source_code)?;

        // Parse impl blocks
        contract.impl_blocks = Self::extract_impl_blocks(source_code)?;

        Ok(contract)
    }

    fn extract_contract_name(source_code: &str, file_path: &str) -> anyhow::Result<String> {
        // Try to find contract name from struct impl
        let contract_regex = regex::Regex::new(r"impl\s+(\w+)\s*\{")?;
        if let Some(captures) = contract_regex.captures(source_code) {
            return Ok(captures[1].to_string());
        }

        // Fallback to filename
        if let Some(stem) = std::path::Path::new(file_path).file_stem() {
            if let Some(name) = stem.to_str() {
                return Ok(name.to_string());
            }
        }

        anyhow::bail!("Could not determine contract name")
    }

    fn extract_imports(source_code: &str) -> Vec<String> {
        let import_regex = regex::Regex::new(r"use\s+([^;]+);").unwrap();
        import_regex
            .captures_iter(source_code)
            .map(|caps| caps[1].trim().to_string())
            .collect()
    }

    fn extract_functions(source_code: &str) -> anyhow::Result<Vec<Function>> {
        let mut functions = Vec::new();
        
        // Function regex pattern
        let function_regex = regex::Regex::new(
            r"(?s)(?P<attributes>#\[.*?\]\s*)*(?P<visibility>pub\s+|fn\s+)(?P<unsafe>unsafe\s+)?fn\s+(?P<name>\w+)\s*(?P<generics<[^>]*>)?\s*\((?P<params>[^)]*)\)\s*(?P<return_type>->\s*[^{]+)?\s*\{(?P<body>.*?)\}"
        ).unwrap();

        for (line_num, caps) in function_regex.captures_iter(source_code).enumerate() {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };

            let is_mut = caps["name"].contains("mut") || 
                        caps["attributes"].contains("#[contracttype]") ||
                        caps["attributes"].contains("#[contractimpl]");

            let parameters = Self::parse_parameters(&caps["params"])?;
            let return_type = if let Some(ret) = caps.name("return_type") {
                Some(Self::parse_type(&ret.as_str()[2..].trim())?) // Skip "-> "
            } else {
                None
            };

            let body_lines: Vec<String> = caps["body"]
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();

            let attributes = Self::parse_attributes(&caps["attributes"])?;

            let generics = if let Some(generics) = caps.name("generics") {
                Self::extract_generic_params(&generics.as_str())
            } else {
                Vec::new()
            };

            functions.push(Function {
                name,
                visibility,
                is_mut,
                parameters,
                return_type,
                body: body_lines,
                line_start: line_num + 1,
                line_end: line_num + 1, // Simplified for now
                attributes,
                generics,
                where_clause: None,
            });
        }

        Ok(functions)
    }

    fn extract_structs(source_code: &str) -> anyhow::Result<Vec<Struct>> {
        let mut structs = Vec::new();
        
        let struct_regex = regex::Regex::new(
            r"(?s)(?P<attributes>#\[.*?\]\s*)*(?P<visibility>pub\s+)?struct\s+(?P<name>\w+)\s*(?P<generics<[^>]*>)?\s*\{(?P<body>.*?)\}"
        ).unwrap();

        for (line_num, caps) in struct_regex.captures_iter(source_code).enumerate() {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };

            let fields = Self::parse_fields(&caps["body"])?;
            let attributes = Self::parse_attributes(&caps["attributes"])?;

            let generics = if let Some(generics) = caps.name("generics") {
                Self::extract_generic_params(&generics.as_str())
            } else {
                Vec::new()
            };

            structs.push(Struct {
                name,
                fields,
                visibility,
                line_start: line_num + 1,
                line_end: line_num + 1,
                attributes,
                generics,
            });
        }

        Ok(structs)
    }

    fn extract_enums(source_code: &str) -> anyhow::Result<Vec<Enum>> {
        let mut enums = Vec::new();
        
        let enum_regex = regex::Regex::new(
            r"(?s)(?P<attributes>#\[.*?\]\s*)*(?P<visibility>pub\s+)?enum\s+(?P<name>\w+)\s*(?P<generics<[^>]*>)?\s*\{(?P<body>.*?)\}"
        ).unwrap();

        for (line_num, caps) in enum_regex.captures_iter(source_code).enumerate() {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };

            let variants = Self::parse_variants(&caps["body"])?;
            let attributes = Self::parse_attributes(&caps["attributes"])?;

            let generics = if let Some(generics) = caps.name("generics") {
                Self::extract_generic_params(&generics.as_str())
            } else {
                Vec::new()
            };

            enums.push(Enum {
                name,
                variants,
                visibility,
                line_start: line_num + 1,
                line_end: line_num + 1,
                attributes,
                generics,
            });
        }

        Ok(enums)
    }

    fn extract_constants(source_code: &str) -> anyhow::Result<Vec<Constant>> {
        let mut constants = Vec::new();
        
        let const_regex = regex::Regex::new(
            r"(?s)(?P<visibility>pub\s+)?const\s+(?P<name>\w+)\s*:\s*(?P<type>[^=]+)\s*=\s*(?P<value>[^;]+);"
        ).unwrap();

        for (line_num, caps) in const_regex.captures_iter(source_code).enumerate() {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };

            let const_type = Self::parse_type(&caps["type"].trim())?;
            let value = caps["value"].trim().to_string();

            constants.push(Constant {
                name,
                const_type,
                value,
                visibility,
                line: line_num + 1,
            });
        }

        Ok(constants)
    }

    fn extract_type_aliases(source_code: &str) -> anyhow::Result<Vec<TypeAlias>> {
        let mut type_aliases = Vec::new();
        
        let type_alias_regex = regex::Regex::new(
            r"(?s)(?P<visibility>pub\s+)?type\s+(?P<name>\w+)\s*=\s*(?P<target_type>[^;]+);"
        ).unwrap();

        for (line_num, caps) in type_alias_regex.captures_iter(source_code).enumerate() {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };

            let target_type = Self::parse_type(&caps["target_type"].trim())?;

            type_aliases.push(TypeAlias {
                name,
                target_type,
                visibility,
                line: line_num + 1,
            });
        }

        Ok(type_aliases)
    }

    fn extract_traits(source_code: &str) -> anyhow::Result<Vec<Trait>> {
        let mut traits = Vec::new();
        
        let trait_regex = regex::Regex::new(
            r"(?s)(?P<attributes>#\[.*?\]\s*)*(?P<visibility>pub\s+)?trait\s+(?P<name>\w+)\s*(?P<generics<[^>]*>)?\s*\{(?P<body>.*?)\}"
        ).unwrap();

        for (line_num, caps) in trait_regex.captures_iter(source_code).enumerate() {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };

            let methods = Self::extract_functions(&caps["body"])?;
            let attributes = Self::parse_attributes(&caps["attributes"])?;

            let generics = if let Some(generics) = caps.name("generics") {
                Self::extract_generic_params(&generics.as_str())
            } else {
                Vec::new()
            };

            traits.push(Trait {
                name,
                methods,
                line_start: line_num + 1,
                line_end: line_num + 1,
                attributes,
                generics,
            });
        }

        Ok(traits)
    }

    fn extract_impl_blocks(source_code: &str) -> anyhow::Result<Vec<ImplBlock>> {
        let mut impl_blocks = Vec::new();
        
        let impl_regex = regex::Regex::new(
            r"(?s)impl\s+(?P<generics<[^>]*>)?\s*(?P<trait>\w+\s+for\s+)?(?P<target_type>\w+)\s*(?P<where_clause>where\s+[^{]+)?\s*\{(?P<body>.*?)\}"
        ).unwrap();

        for (line_num, caps) in impl_regex.captures_iter(source_code).enumerate() {
            let target_type = Self::parse_type(&caps["target_type"])?;
            let trait_name = caps.name("trait")
                .map(|m| m.as_str().trim().replace(" for", ""));

            let methods = Self::extract_functions(&caps["body"])?;

            let generics = if let Some(generics) = caps.name("generics") {
                Self::extract_generic_params(&generics.as_str())
            } else {
                Vec::new()
            };

            let where_clause = caps.name("where_clause")
                .map(|m| m.as_str().trim().to_string());

            impl_blocks.push(ImplBlock {
                target_type,
                trait_name,
                methods,
                line_start: line_num + 1,
                line_end: line_num + 1,
                generics,
            });
        }

        Ok(impl_blocks)
    }

    fn parse_parameters(params_str: &str) -> anyhow::Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        
        if params_str.trim().is_empty() {
            return Ok(parameters);
        }

        let param_regex = regex::Regex::new(
            r"(?P<is_mut>mut\s+)?(?P<name>\w+)\s*:\s*(?P<type>[^,]+)"
        ).unwrap();

        for caps in param_regex.captures_iter(params_str) {
            let name = caps["name"].to_string();
            let is_mut = caps.name("is_mut").is_some();
            let param_type = Self::parse_type(&caps["type"].trim())?;

            parameters.push(Parameter {
                name,
                param_type,
                is_mut,
            });
        }

        Ok(parameters)
    }

    fn parse_fields(fields_str: &str) -> anyhow::Result<Vec<Field>> {
        let mut fields = Vec::new();
        
        let field_regex = regex::Regex::new(
            r"(?P<visibility>pub\s+)?(?P<name>\w+)\s*:\s*(?P<type>[^,]+)"
        ).unwrap();

        for caps in field_regex.captures_iter(fields_str) {
            let name = caps["name"].to_string();
            let visibility = if caps["visibility"].starts_with("pub") {
                Visibility::Public
            } else {
                Visibility::Private
            };
            let field_type = Self::parse_type(&caps["type"].trim())?;

            fields.push(Field {
                name,
                field_type,
                visibility,
            });
        }

        Ok(fields)
    }

    fn parse_variants(variants_str: &str) -> anyhow::Result<Vec<Variant>> {
        let mut variants = Vec::new();
        
        let variant_regex = regex::Regex::new(
            r"(?s)(?P<attributes>#\[.*?\]\s*)?(?P<name>\w+)\s*(?P<fields>\([^)]*\))?\s*(?P<struct_fields>\{[^}]*\})?"
        ).unwrap();

        for caps in variant_regex.captures_iter(variants_str) {
            let name = caps["name"].to_string();
            let attributes = Self::parse_attributes(&caps["attributes"])?;

            let mut fields = Vec::new();
            
            // Handle tuple-like variants
            if let Some(tuple_fields) = caps.name("fields") {
                let fields_str = &tuple_fields.as_str()[1..tuple_fields.as_str().len()-1]; // Remove parentheses
                if !fields_str.trim().is_empty() {
                    for field_str in fields_str.split(',') {
                        fields.push(Self::parse_type(field_str.trim())?);
                    }
                }
            }
            
            // Handle struct-like variants
            if let Some(struct_fields) = caps.name("struct_fields") {
                let fields_str = &struct_fields.as_str()[1..struct_fields.as_str().len()-1]; // Remove braces
                for field_str in fields_str.split(',') {
                    if let Some(colon_pos) = field_str.find(':') {
                        let type_str = field_str[colon_pos+1..].trim();
                        fields.push(Self::parse_type(type_str)?);
                    }
                }
            }

            variants.push(Variant {
                name,
                fields,
                attributes,
            });
        }

        Ok(variants)
    }

    fn parse_attributes(attributes_str: &str) -> anyhow::Result<Vec<Attribute>> {
        let mut attributes = Vec::new();
        
        if attributes_str.trim().is_empty() {
            return Ok(attributes);
        }

        let attr_regex = regex::Regex::new(r"#\[(?P<name>\w+)(?P<args>\([^)]*\))?\]").unwrap();

        for caps in attr_regex.captures_iter(attributes_str) {
            let name = caps["name"].to_string();
            let args = if let Some(args) = caps.name("args") {
                let args_str = &args.as_str()[1..args.as_str().len()-1]; // Remove parentheses
                args_str.split(',').map(|s| s.trim().to_string()).collect()
            } else {
                Vec::new()
            };

            attributes.push(Attribute {
                name,
                args,
                line: 0, // Would need more context for accurate line numbers
            });
        }

        Ok(attributes)
    }

    fn parse_type(type_str: &str) -> anyhow::Result<Type> {
        let type_str = type_str.trim();
        
        // Handle simple types
        if !type_str.contains('<') && !type_str.contains('(') && !type_str.contains('[') {
            return Ok(Type::Simple(type_str.to_string()));
        }

        // Handle generic types
        if let Some(angle_start) = type_str.find('<') {
            if let Some(angle_end) = type_str.rfind('>') {
                let name = type_str[..angle_start].trim().to_string();
                let args_str = &type_str[angle_start+1..angle_end];
                let mut args = Vec::new();
                
                for arg_str in args_str.split(',') {
                    args.push(Self::parse_type(arg_str.trim())?);
                }
                
                return Ok(Type::Generic { name, args });
            }
        }

        // Handle tuples
        if type_str.starts_with('(') && type_str.ends_with('') {
            let inner = &type_str[1..type_str.len()-1];
            let mut types = Vec::new();
            
            for type_part in inner.split(',') {
                types.push(Self::parse_type(type_part.trim())?);
            }
            
            return Ok(Type::Tuple(types));
        }

        // Handle arrays/slices
        if type_str.starts_with('[') {
            if let Some(bracket_end) = type_str.find(']') {
                let element_type_str = &type_str[1..bracket_end];
                let element_type = Self::parse_type(element_type_str.trim())?;
                
                let remaining = &type_str[bracket_end+1..];
                if remaining.starts_with(';') {
                    let size = remaining[1..].trim().to_string();
                    return Ok(Type::Array { 
                        element_type: Box::new(element_type), 
                        size: Some(size) 
                    });
                } else {
                    return Ok(Type::Slice(Box::new(element_type)));
                }
            }
        }

        // Handle references
        if type_str.starts_with('&') {
            let remaining = &type_str[1..].trim();
            let is_mut = remaining.starts_with("mut ");
            let target_str = if is_mut { &remaining[4..] } else { remaining };
            let target_type = Self::parse_type(target_str)?;
            
            return Ok(Type::Reference { 
                target: Box::new(target_type), 
                is_mut 
            });
        }

        // Fallback to simple type
        Ok(Type::Simple(type_str.to_string()))
    }

    fn extract_generic_params(generics_str: &str) -> Vec<String> {
        let inner = &generics_str[1..generics_str.len()-1]; // Remove < and >
        inner
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}
