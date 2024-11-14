/// https://practice.course.rs/variables.html
#[test]
fn test36() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let mut _x = x;
    x += 3;


    let y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}

#[test]
fn test39() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}