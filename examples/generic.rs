struct Point<T, U> {
    x: T,
    y: U,
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        let n = 2;
        (self.x.powi(n) + self.y.powi(n)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    let p = Point { x: 15, y: 20.02 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p1: Point<i32, f64> = Point { x: 5, y: 10.4 };
    let p2: Point<&str, char> = Point { x: "Hello", y: 'c' };
    let p3: Point<i32, char> = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
