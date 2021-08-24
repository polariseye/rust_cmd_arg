//! Command Line argument parser.
//! the code is base on https://github.com/polariseye/cmdpro.
//!
//! #examples
//! ```
//! // for this example.cmd line is: test --path ./hello.txt --value hello
//! // and also it can be: test -p ./hello.txt -v hello
//! // and also it can be: test /path ./hello.txt /value hello
//! use rust_cmd_arg::{CommandLineProcessor,ParameterType,ParameterValue};
//! fn main() {
//! 	// Create a new CommandLineProcessor
//!     let mut command_line_processor = CommandLineProcessor::new();
//!
//!     // Add Parameters
//!     let path_param = command_line_processor.add_parameter_detail("path", ParameterType::Path,false,ParameterValue::None,"file path desc" , vec!["-p".to_owned()]);
//!     let value_param = command_line_processor.add_parameter_detail("value", ParameterType::UInteger,false,ParameterValue::None,"value desc", vec!["-v".to_owned()]);
//!
//!     // Parse the command line parameters
//!     command_line_processor.parse_command_line();
//!
//! 	// Print the parameter values
//!     println!("Path: {:?}", path_param.to_path_value().expect("wrong value for path").to_str().unwrap());
//!     println!("Value: {}", value_param.to_string_value().expect("wrong value for value"));
//! }
//! ```

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;

/// List of parameter types that can be processed.
#[derive(Debug)]
pub enum ParameterType {
    /// Flag parameter.
    Flag,

    /// integer Value.
    Integer,

    /// float value
    Float,

    /// File Path.
    Path,

    /// string value
    String,

    /// bool value
    Bool,
}

/// `ParameterType` with its assigned value.
#[derive(Debug, Clone)]
pub enum ParameterValue {
    /// No value.
    None,

    /// Flag parameter has been set.
    Flag,

    /// i64 Value
    Integer(i64),

    /// float value
    Float(f64),

    /// File Path.
    Path(PathBuf),

    /// string value
    String(String),

    /// bool value
    Bool(bool),
}

impl ParameterValue {
    pub fn is_none(&self) -> bool {
        return match self {
            ParameterValue::None => true,
            _ => false,
        };
    }
    pub fn to_int_value(&self) -> Result<i64, String> {
        return match self {
            ParameterValue::Integer(val) => Ok(*val),
            _ => Err(format!("wrong value type:{:?}", self)),
        };
    }
    pub fn to_float_value(&self) -> Result<f64, String> {
        return match self {
            ParameterValue::Float(val) => Ok(*val),
            _ => Err(format!("wrong value type:{:?}", self)),
        };
    }
    pub fn to_path_value(&self) -> Result<PathBuf, String> {
        return match self {
            ParameterValue::Path(val) => Ok(val.clone()),
            _ => Err(format!("wrong value type:{:?}", self)),
        };
    }
    pub fn to_string_value(&self) -> Result<String, String> {
        return match self {
            ParameterValue::String(val) => Ok(val.to_string()),
            _ => Err(format!("wrong value type:{:?}", self)),
        };
    }
    pub fn to_bool_value(&self) -> Result<bool, String> {
        return match self {
            ParameterValue::Bool(val) => Ok(*val),
            _ => Err(format!("wrong value type:{:?}", self)),
        };
    }

    pub fn to_help_string(&self) -> String {
        return match self {
            ParameterValue::None => "".to_string(),
            ParameterValue::Flag => true.to_string(),
            ParameterValue::Integer(val) => format!("{}", val),
            ParameterValue::Float(val) => format!("{}", val),
            ParameterValue::Path(val) => format!("{}", val.to_str().unwrap()),
            ParameterValue::String(val) => format!("{}", val),
            ParameterValue::Bool(val) => format!("{}", val),
        };
    }
}

pub struct Parameter {
    parameter_name: String,
    parameter_type: ParameterType,
    allow_empty: bool,
    aliases: Vec<String>,
    description: String,
    default_value: ParameterValue,
    value: RefCell<ParameterValue>,
}

impl Parameter {
    pub fn get_value(&self) -> Ref<ParameterValue> {
        return self.value.borrow();
    }

    pub fn to_int_value(&self) -> Result<i64, String> {
        let val = self.value.borrow();
        if val.is_none() {
            if self.allow_empty {
                return Ok(0);
            }

            return Err(format!("{} is None Value", &self.parameter_name));
        }

        val.to_int_value()
    }
    pub fn to_float_value(&self) -> Result<f64, String> {
        let val = self.value.borrow();
        if val.is_none() {
            if self.allow_empty {
                return Ok(0f64);
            }

            return Err(format!("{} is None Value", &self.parameter_name));
        }

        val.to_float_value()
    }
    pub fn to_path_value(&self) -> Result<PathBuf, String> {
        let val = self.value.borrow();
        if val.is_none() {
            if self.allow_empty {
                return Ok(PathBuf::from(""));
            }

            return Err(format!("{} is None Value", &self.parameter_name));
        }

        val.to_path_value()
    }
    pub fn to_string_value(&self) -> Result<String, String> {
        let val = self.value.borrow();
        if val.is_none() {
            if self.allow_empty {
                return Ok("".to_string());
            }

            return Err(format!("{} is None Value", &self.parameter_name));
        }

        val.to_string_value()
    }
    pub fn to_bool_value(&self) -> Result<bool, String> {
        let val = self.value.borrow();
        if val.is_none() {
            if self.allow_empty {
                return Ok(false);
            }

            return Err(format!("{} is None Value", &self.parameter_name));
        }

        val.to_bool_value()
    }
}

/// Command Line Processor
pub struct CommandLineProcessor {
    parameters: HashMap<String, Rc<Parameter>>,
    version_text: Option<String>,
    abort_flag: bool,
}

impl CommandLineProcessor {
    /// Returns a new `CommandLineProcessor`.
    pub fn new() -> CommandLineProcessor {
        CommandLineProcessor {
            parameters: HashMap::new(),
            version_text: None,
            abort_flag: false,
        }
    }

    /// Add a parameter to be parsed.
    pub fn add_parameter_detail(
        &mut self,
        parameter_name: &str,
        parameter_type: ParameterType,
        allow_empty: bool,
        default_value: ParameterValue,
        description: &str,
        mut aliases: Vec<String>,
    ) -> Rc<Parameter> {
        let alias1 = "/".to_string() + parameter_name;
        let alias2 = "--".to_string() + parameter_name;
        if aliases.iter().any(|item| item == &alias1) == false {
            aliases.push(alias1)
        }
        if aliases.iter().any(|item| item == &alias2) == false {
            aliases.push(alias2)
        }

        let parameter = Rc::new(Parameter {
            parameter_name: parameter_name.to_owned(),
            parameter_type,
            aliases,
            allow_empty,
            description: description.to_string(),
            default_value: default_value.clone(),
            value: RefCell::new(default_value),
        });

        self.parameters
            .insert(parameter_name.to_owned(), parameter.clone());

        parameter
    }

    /// 添加参数
    pub fn add_simple_parameter(
        &mut self,
        parameter_name: &str,
        parameter_type: ParameterType,
        description: &str,
    ) -> Rc<Parameter> {
        self.add_parameter_detail(
            parameter_name,
            parameter_type,
            false,
            ParameterValue::None,
            description,
            vec![],
        )
    }

    /// 添加参数
    pub fn add_can_empty_parameter(
        &mut self,
        parameter_name: &str,
        parameter_type: ParameterType,
        default_value: ParameterValue,
        description: &str,
    ) -> Rc<Parameter> {
        self.add_parameter_detail(
            parameter_name,
            parameter_type,
            true,
            default_value,
            description,
            vec![],
        )
    }
    /// Parses the program's command line parameters.
    ///
    /// # Panics
    /// Panics if the parameter type requires a value and no value is provided.
    /// It will also panic if the parameter is the wrong type.
    pub fn parse_command_line(&mut self) {
        let mut iter = env::args();
        iter.next(); // Skip executable name

        loop {
            match iter.next() {
                Some(argument) => match argument.as_ref() {
                    "--help" => {
                        self.print_help_text();
                        self.abort_flag = true;
                        break;
                    }
                    "--h" => {
                        self.print_help_text();
                        self.abort_flag = true;
                        break;
                    }
                    "--version" => {
                        self.print_version_text();
                        self.abort_flag = true;
                        break;
                    }
                    "--v" => {
                        self.print_version_text();
                        self.abort_flag = true;
                        break;
                    }
                    arg => {
                        let mut parameter_exists = false;

                        for (name, parameter) in self.parameters.iter_mut() {
                            if parameter.aliases.iter().any(|x| x == arg) {
                                parameter_exists = true;

                                match parameter.parameter_type {
                                    ParameterType::Flag => {
                                        *parameter.value.borrow_mut() = ParameterValue::Flag
                                    }
                                    ParameterType::Integer => match iter.next() {
                                        Some(val) => {
                                            if val.is_empty() {
                                                if parameter.allow_empty == false {
                                                    println!(
                                                        "No value passed for parameter {}",
                                                        name
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                            }

                                            match val.parse::<i64>() {
                                                Ok(val) => {
                                                    *parameter.value.borrow_mut() =
                                                        ParameterValue::Integer(val)
                                                }
                                                Err(err) => {
                                                    println!(
                                                        "Unable to convert parameter {} to integer\n{}",
                                                        name, err
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                            }
                                        }
                                        None => {
                                            if parameter.allow_empty == false {
                                                println!("No value passed for parameter {}", name);
                                                self.abort_flag = true;
                                                break;
                                            }
                                        }
                                    },
                                    ParameterType::Float => match iter.next() {
                                        Some(val) => {
                                            if val.is_empty() {
                                                if parameter.allow_empty == false {
                                                    println!(
                                                        "No value passed for parameter {}",
                                                        name
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                                continue;
                                            }

                                            match val.parse::<f64>() {
                                                Ok(val) => {
                                                    *parameter.value.borrow_mut() =
                                                        ParameterValue::Float(val)
                                                }
                                                Err(err) => {
                                                    println!(
                                                        "Unable to convert parameter {} to float\n{}",
                                                        name, err
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                            }
                                        }
                                        None => {
                                            if parameter.allow_empty == false {
                                                panic!("No value passed for parameter {}", name)
                                            }
                                        }
                                    },
                                    ParameterType::String => match iter.next() {
                                        Some(val) => {
                                            if val.is_empty() {
                                                if parameter.allow_empty == false {
                                                    println!(
                                                        "No value passed for parameter {}",
                                                        name
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                                continue;
                                            }

                                            *parameter.value.borrow_mut() =
                                                ParameterValue::String(val)
                                        }
                                        None => {
                                            if parameter.allow_empty == false {
                                                println!("No value passed for parameter {}", name);
                                                self.abort_flag = true;
                                                break;
                                            }
                                        }
                                    },
                                    ParameterType::Bool => match iter.next() {
                                        Some(val) => {
                                            if val.is_empty() {
                                                if parameter.allow_empty == false {
                                                    println!(
                                                        "No value passed for parameter {}",
                                                        name
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                                continue;
                                            }

                                            match val.parse::<bool>() {
                                                Ok(val) => {
                                                    *parameter.value.borrow_mut() =
                                                        ParameterValue::Bool(val)
                                                }
                                                Err(err) => {
                                                    println!(
                                                        "Unable to convert parameter {} to bool\n{}",
                                                        name, err
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                            }
                                        }
                                        None => {
                                            if parameter.allow_empty == false {
                                                println!("No value passed for parameter {}", name);
                                                self.abort_flag = true;
                                                break;
                                            }
                                        }
                                    },
                                    ParameterType::Path => match iter.next() {
                                        Some(val) => {
                                            if val.is_empty() {
                                                if parameter.allow_empty == false {
                                                    println!(
                                                        "No value passed for parameter {}",
                                                        name
                                                    );
                                                    self.abort_flag = true;
                                                    break;
                                                }
                                                continue;
                                            }

                                            let mut path = PathBuf::new();
                                            path.push(val);
                                            *parameter.value.borrow_mut() =
                                                ParameterValue::Path(path);
                                        }
                                        None => {
                                            if parameter.allow_empty == false {
                                                println!("No value passed for parameter {}", name);
                                                self.abort_flag = true;
                                                break;
                                            }
                                        }
                                    },
                                }
                            }
                        }

                        if !parameter_exists {
                            println!("Unknown parameter: {}", arg);
                            self.abort_flag = true;
                            break;
                        }
                    }
                },
                None => break,
            }
        }

        if self.abort_flag {
            self.print_help_text();
            std::process::exit(-1);
        }
        if self.check_if_parse_all_arg() == false {
            self.print_help_text();
            std::process::exit(-2);
        }
    }

    fn check_if_parse_all_arg(&mut self) -> bool {
        let mut if_parse_all_arg = true;
        for item in self.parameters.values() {
            if item.allow_empty {
                continue;
            }

            if item.value.borrow().is_none() {
                println!("cmd arg {} is no set", &item.parameter_name);
                self.abort_flag = true;
                if_parse_all_arg = false;
            }
        }

        if_parse_all_arg
    }

    /// Print the default help text
    fn print_help_text(&self) {
        println!(
            "USAGE \r\n\t{} [OPTIONS]\r\n",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("OPTIONS");

        let mut param_str_list: Vec<Vec<String>> = vec![];
        param_str_list.push(vec![
            "arg".to_string(),
            "IsCanEmpty".to_string(),
            "DefaultValue".to_string(),
            "Description".to_string(),
        ]);
        for item in self.parameters.values() {
            // name[alias1,alias2] can empty default value description

            let arg_name = format!("{}", item.aliases.join(","));
            let mut can_empty = "false";
            if item.allow_empty {
                can_empty = "true";
            }

            let default_value = item.default_value.to_help_string();
            param_str_list.push(vec![
                arg_name,
                can_empty.to_string(),
                default_value.to_string(),
                item.description.to_string(),
            ]);
        }

        // calculate width
        let mut col_max_width: [usize; 4] = [0, 0, 0, 0];
        for arg_item in &param_str_list {
            for col_index in 0..arg_item.len() {
                let tmp_len = arg_item[col_index].len();
                if tmp_len > col_max_width[col_index] {
                    col_max_width[col_index] = tmp_len;
                }
            }
        }

        // print
        for arg_item in &param_str_list {
            println!("\t{name:name_width$}\t{can_empty:can_empty_width$}\t{default_value:default_value_width$}\t{description:description_width$}",
                     name=arg_item[0],name_width=col_max_width[0]
                     ,can_empty=arg_item[1],can_empty_width=col_max_width[1]
                     ,default_value=arg_item[2],default_value_width=col_max_width[2]
                     ,description=arg_item[3],description_width=col_max_width[3])
        }
    }

    /// Sets the text to print when the `--version` parameter is used.
    pub fn set_version_text(&mut self, version_text: &str) {
        self.version_text = Some(version_text.to_owned());
    }

    /// Prints the version text. Prints a default message if the version text is not set.
    fn print_version_text(&self) {
        match &self.version_text {
            Some(version_text) => println!("{}", version_text),
            None => println!("No version text has been set."),
        }
    }

    /// Returns the `ParameterValue` for the specified parameter. Returns `ParameterValue::None` if the parameter doesn't exist.
    pub fn get_parameter_value(&self, parameter_name: &str) -> Option<Ref<ParameterValue>> {
        match self.parameters.get(parameter_name) {
            Some(parameter) => Some(parameter.value.borrow()),
            None => None,
        }
    }

    /// Returns true if the `CommandLineProcessor` reads `--help` or `--version` in the parameter list.
    pub fn abort_flag(&self) -> bool {
        self.abort_flag
    }
}
