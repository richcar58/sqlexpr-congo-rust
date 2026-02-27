use parser::{Parser, ParseError};


fn main() -> Result<(), ParseError>{
    println!("Parsing...");

    let input = "b > 5";
    //let input = "4 * 6 + 7 / 8".to_string();
    let mut parser = Parser::new(input.to_string()).unwrap();
    let result = parser.parse();
    println!("Result: {:?}", result);   
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());

    // Parse returns root node ID
    let root_id = result.unwrap();

    // Access nodes through the arena
    let arena = parser.arena();
    let ast = arena.pretty_print(root_id, 0, parser.input());
    println!("{}", ast);

    // println!("Parsing completed successfully.");
    Ok(())
}


