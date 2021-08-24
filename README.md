# rust_cmd_arg
this code is base on [cmdpro](https://github.com/polariseye/cmdpro). but rust_cmd_arg is easier to use. for example
```
use rust_cmd_arg::{CommandLineProcessor,ParameterType,ParameterValue};
fn main() {
	// Create a new CommandLineProcessor
    let mut command_line_processor = CommandLineProcessor::new();

    // Add Parameters
    let path_param = command_line_processor.add_parameter_detail("path", ParameterType::Path,false,ParameterValue::None,"file path desc" , vec!["-p".to_owned()]);
    let value_param = command_line_processor.add_parameter_detail("value", ParameterType::UInteger,false,ParameterValue::None,"value desc", vec!["-v".to_owned()]);

    // Parse the command line parameters
    command_line_processor.parse_command_line();

	// Print the parameter values
    println!("Path: {:?}", path_param.to_path_value().expect("wrong value for path").to_str().unwrap());
    println!("Value: {}", value_param.to_string_value().expect("wrong value for value"));
```