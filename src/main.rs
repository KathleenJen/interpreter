<<<<<<< HEAD
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
=======
// #[derive(Copy, Clone)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Number(i32)
}

pub fn evaluate_addition(expression: &Expression) -> i32 {
    if let Expression::Add(expressions) = expression {
        let iter = expressions.iter();
        iter.fold(0, |total, next| total + evaluate(next))
    } else {
        panic!("addition not provided")
    }
}

pub fn evaluate_multiply(expression: &Expression) -> i32 {
    if let Expression::Multiply(expressions) = expression {
        let iter = expressions.iter();
        iter.fold(1, |total, next| total * evaluate(next))
    } else {
        panic!("addition not provided")
    }
}

pub fn evaluate(expression: &Expression) -> i32 {
    match expression {
        Expression::Add(_) => evaluate_addition(expression),
        Expression::Multiply(_) => evaluate_multiply(expression),
        Expression::Number(val) => *val
>>>>>>> 99f2cafa7b05b59c5ae438aa46c38a98603c46d4
    }
}

fn main() {
<<<<<<< HEAD
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Subtract);
    primitives.push(Primitive::Number(7));
    primitives.push(Primitive::Number(1));
    let result = evaluate(primitives);
    println!("{}", result);
=======
    let addition = Expression::Add(vec![Expression::Number(2), Expression::Number(2)]);
    println!("2 + 2 is {}", evaluate(&addition));
>>>>>>> 99f2cafa7b05b59c5ae438aa46c38a98603c46d4
}

// Arrange
// Act
// Assert

// Given
// When
// Then

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_addition() {
        // Arrange
        let three = crate::Expression::Number(3);
        let four = crate::Expression::Number(4);
        let addition = crate::Expression::Add(vec![three, four]);
        // Act
        let sum = crate::evaluate_addition(&addition);
        // Assert
        assert_eq!(sum, 7);
    }

    #[test]
    fn test_multiply() {
        // Arrange
        let three = crate::Expression::Number(3);
        let four = crate::Expression::Number(4);
        let multiplication = crate::Expression::Multiply(vec![three, four]);
        // Act
        let product = crate::evaluate_multiply(&multiplication);
        // Assert
        assert_eq!(product, 12);
    }
}