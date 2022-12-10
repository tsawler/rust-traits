// define a trait Fly. In order to implement this Trait, 
// a type must have all of the functions listed (fly and print_name).
trait Fly {
    fn fly(&self);
    fn return_name(&self) -> String;
}

// Define a type Bird, with its own fields. We use a macro to implement clone for this type.
#[derive(Clone)]
struct Bird<'a> {
    name: &'a str,  // 'a is a lifetime ellision
    wing_span: f32,
    unit: &'a str,
}

// implement the trait Fly for the type bird, by implementing
// all of the required functions.
impl Fly for Bird<'_> {
    fn fly(&self) {
        println!("{} flaps his wings and flies for a couple of inches...", self.name)
    }
    fn return_name(&self) -> String{
        let my_name = &self.name;
        return my_name.to_string();
    }
}


// We can also implement functions for the type Bird that are unrelated
// to the Fly type.
impl Bird<'_> {
    fn how_wide_are_my_wings(&self) {
        println!("{}'s wingspan is {} {}.", self.name, self.wing_span, self.unit)
    }
}

// define the Plan type, with it's own fields.
struct Plane {
    model: String,
    range: usize,
}


impl Fly for Plane {
    fn fly(&self) {
        println!("The {} takes off and flies for {} km before it has to land.", self.model, self.range)
    }
    fn return_name(&self) -> String {
        let my_name = &self.model;
        return my_name.to_string()
    }
}

fn main() {
    // create a variable of type Bird
    let mut tweety = Bird {
        name: "Tweety", 
        wing_span: 3.3,
        unit: "inches",
    };

    // because bird implements the Fly trait, the variable tweety must 
    // have a fly() function.
    tweety.fly();

    // we can also call its other functions, if any:
    tweety.how_wide_are_my_wings();

    // create  variable of type Plane.
    let mut bell_ranger = Plane {
        model: "Bell Ranger".to_string(), 
        range: 750
    };

    // since the variable bell_ranger implements the Fly trait, it must have 
    // the function fly() as well.
    bell_ranger.fly();

    what_am_i(&mut tweety);
    what_am_i(&mut bell_ranger);

    // print out the wingspan for tweenty
    println!("In the main func, before call to receive bird, Tweety's wingspan is {}", tweety.wing_span);

    // call functions that only accept one type
    receive_bird(&mut tweety);

    receive_plane(bell_ranger)
}


// This function accepts anything that implements Fly
fn what_am_i<T: Fly>(item: &mut T) {
    println!("What am I? I'm a {}.", String::from(item.return_name()))
}

// This function accepts only Bird
fn receive_bird(b: &mut Bird) {
    println!("The next line comes from the receive bird function:");

    // change wingspan
    b.wing_span = 3.34;

    // call the function on the var
    b.how_wide_are_my_wings()
}

fn receive_plane(p: Plane) {
    println!("The next line comes from teh receive_plane function:");
    p.fly()
}
