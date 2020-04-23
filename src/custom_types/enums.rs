// enums declaration similar to other oop languages
enum Number {
    Zero,
    One,
    Two
}
// enums in rust are completely different from other oop languages in some cases
// to make it easier to understand, consider enums as a skeleton or declaration where while implementing or consuming enums we can perform certain operations based on inputs but it can't return anything
// as shown below SomeEvent is an Enum which can also take input params or can have some properties
enum SomeEvent {
    FormLoad,
    FormUnload,
    KeyUp(char),
    Clipboard(String),
    MousePointer { x: i32, y: i32 },
}

// below is the implementation of above enum, where we are defining set of operation to be performed based on inputs or without any input
// key point to note here is the below implementation method shouldn't have any return type
fn consume_event(event: SomeEvent) {
    match event {
        SomeEvent::FormLoad => println!("Form load event called"),
        SomeEvent::FormUnload => println!("Form unload event called"),
        SomeEvent::KeyUp(c) => println!("Key up called with key: '{}'", c),
        SomeEvent::Clipboard(content) => println!("Content from clipboard is: '{}'", content),
        SomeEvent::MousePointer { x, y } => println!("Current mouse pointer are x:{}, y:{}", x, y),
    }
}

pub fn access_enums() {
    println!("enums first element: {}", Number::Zero as i32);
    println!("enums second element: {}", Number::One as i32);
    println!("enums third element: {}", Number::Two as i32);

    consume_event(SomeEvent::FormLoad);
    consume_event(SomeEvent::FormUnload);
    consume_event(SomeEvent::KeyUp('x'));
    consume_event(SomeEvent::Clipboard(String::from("clipboard content")));
    consume_event(SomeEvent::MousePointer { x: 10, y: 20 });

}
