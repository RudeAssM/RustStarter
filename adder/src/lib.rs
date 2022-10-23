pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a:i32) -> i32{
    a + 2
    //a + 3
}

pub fn greeting(name: &str) -> String{
    format!("hello {}!", name)
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) ->bool{
        self.width > other.width && self.height > other.height
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn another(){
        panic!("aaaaaaaahhhhh")
    }
    */

    #[test]
    fn explore() {
        assert_eq!(2 + 2 , 4);
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 8,
            height: 7,
        };
        let smaller = Rectangle{
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{
            width: 8,
            height: 7,
        };
        let smaller = Rectangle{
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4,add_two(2));
    }

    #[test]
    fn geeting_contains_name(){
        let result = greeting("carol");
        assert!(result.contains("carol"),"greeting didnt contain value '{}'", result);
    }


}
