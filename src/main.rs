// define a trait Fly. In order to implement this Trait, 
// a type must have all of the functions listed (fly and print_name).
trait Fly {
    fn fly(&self);
    fn return_name(&self) -> String;
}

// Define a type Bird, with its own fields.
struct Bird {
    name: String,
    wing_span: f32,
}

// implement the trait Fly for the type bird, by implementing
// all of the required functions.
impl Fly for Bird {
    fn fly(&self) {
        println!("{} flaps his wings", self.name)
    }
    fn return_name(&self) -> String{
        let my_name = &self.name;
        return my_name.to_string();
    }
}

// We can also implement functions for the type Bird that are unrelated
// to the Fly type.
impl Bird {
    fn how_wide_are_my_wings(&self) {
        println!("My wings are {} inches wide.", self.wing_span)
    }
}

struct Plane {
    model: String,
    range: usize,
}


impl Fly for Plane {
    fn fly(&self) {
        println!("A {} can fly for {} km.", self.model, self.range)
    }
    fn return_name(&self) -> String {
        let my_name = &self.model;
        return my_name.to_string()
    }
}

fn main() {
    // create a variable of type Bird
    let tweety = Bird {
        name: String::from("Tweety"), 
        wing_span: 3.3,
    };

    // because bird implements the Fly trait, the variable tweety must 
    // have a fly() function.
    tweety.fly();

    // we can also call its other functions, if any:
    tweety.how_wide_are_my_wings();

    // create  variable of type Plane.
    let bell_ranger = Plane {
        model: "Bell Ranger".to_string(), 
        range: 750
    };

    // since the variable bell_ranger implements the Fly trait, it must have 
    // the function fly() as well.
    bell_ranger.fly();

    what_am_i(tweety);
    what_am_i(bell_ranger);
}


// This function accepts anything that implements Fly
fn what_am_i<T: Fly>(item: T) {
    println!("What am I? I'm a {}.", String::from(item.return_name()))
}


