//Kathleen Jennings
#[derive(Copy, Clone)]
enum Primitive {
    Add,
    Multiply,
    Subtract,
    Number(i32)
}

fn evaluate(array: Vec<Primitive>) -> i32 {
    let element = &array[0];
    let mut iter = array.iter();
    iter.next();
    match element {
        Primitive::Add => { iter.fold(0, |total, next| total + evaluate(vec![*next])) }
        Primitive::Multiply => { iter.fold(1, |total, next| total * evaluate(vec![*next])) }
        //trying to use fold right because subtraction is not comutative, but it does not seem to be working
        Primitive::Subtract => { iter.foldr(0, |total, next| total - evaluate(vec![*next])) }
        Primitive::Number(val) => *val
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Subtract);
    primitives.push(Primitive::Number(7));
    primitives.push(Primitive::Number(1));
    let result = evaluate(primitives);
    println!("{}", result);
}
