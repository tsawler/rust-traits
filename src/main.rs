// define a trait Fly. In order to implement this trait, 
// a type must have all of the functions listed (fly and return_name).
trait Fly {
    fn fly(&self);
    fn return_name(&self) -> String;
}

// Define a type Aircraft, with its own fields.
struct Aircraft {
    model: String,
    range: usize,
    units: String,
}

// Implement the Fly trait for the Aircraft type.
impl Fly for Aircraft {
    fn fly(&self) {
        println!("The {} takes off and flies for {} {} before it has to land.", self.model, self.range, self.units)
    }
    fn return_name(&self) -> String {
        let my_name = &self.model;
        return my_name.to_string()
    }
}

// Define a type Bird, with its own fields. We specify lifetime ellision.
// In the Rust programming language, lifetime elision is a feature that 
// allows the compiler to infer the lifetimes of references in certain situations. 
// This means that in some cases, you don't have to explicitly specify the lifetimes of 
// references in your code, and the compiler will automatically infer the appropriate 
// lifetimes based on the context.
struct Bird<'a> { // 'a is a lifetime ellision
    name: &'a str,  
    wing_span: f32,
    units: &'a str,
}

// implement the trait Fly for the type bird, by implementing
// all of the required functions. Note that we indicate the anonymous lifetime: `<'_>`.
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
// to the Fly type. Note that we indicate the anonymous lifetime: `<'_>`.
impl Bird<'_> {
    fn how_wide_are_my_wings(&self) {
        println!("{}'s wingspan is {} {}.", self.name, self.wing_span, self.units)
    }
}


fn main() {
    // create  variable of type Aircraft.
    let mut bell_ranger = Aircraft {
        model: "Bell Ranger".to_string(), 
        range: 750,
        units: "kilometers".to_string(),
    };

    // Since the variable bell_ranger implements the Fly trait, it must have 
    // the function fly() as well, so we can use it.
    bell_ranger.fly();

    // Create a variable of type Bird:
    let mut tweety = Bird {
        name: "Tweety", 
        wing_span: 3.3,
        units: "inches",
    };

    // Because Bird implements the Fly trait, the variable tweety must 
    // have a fly() function, so we can call it.
    tweety.fly();

    // We can also call its other functions, if any:
    tweety.how_wide_are_my_wings();


    // We can call the what_am_i function with both Aircraft and Bird types, since it
    // accepts anything that implements Fly.
    what_am_i(&mut tweety);
    what_am_i(&mut bell_ranger);

    // Print out the wingspan for tweety:
    println!("In the main func, before call to receive bird, Tweety's wingspan is {} {}", tweety.wing_span, tweety.units);


    // Call a functions that only accept Aircraft.
    receive_aircraft(bell_ranger);

    // Call a functions that only accept Bird. We are changing a field in tweety,
    // so we must pass it using &mut.
    receive_bird(&mut tweety);

    // We can't do something like this, since Aircraft does not implement Copy. In order to make
    // this work, we'd have to do the same thing to Aircraft that we did to 
    // Bird.
    // update_aircraft(&mut bell_ranger, 250)
}


// This function accepts anything that implements Fly.
fn what_am_i<T: Fly>(item: &mut T) {
    println!("What am I? I'm a {}.", String::from(item.return_name()))
}

// This function accepts only Bird.
fn receive_bird(b: &mut Bird) {
    println!("The next line comes from the receive_bird function:");

    // change wingspan
    b.wing_span = 3.34;

    // call the function on the var
    b.how_wide_are_my_wings()
}

// This function only accepts Aircraft.
fn receive_aircraft(p: Aircraft) {
    println!("The next line comes from the receive_aircraft function:");
    p.fly()
}

// this function only exists to show what we can't do, back on line 106.
// It is never called.
// fn update_aircraft(p: &mut Aircraft, r: usize) {
//     p.range = r;
//     println!("The range for {} is now {} {}", p.model, p.range, p.units)
// }