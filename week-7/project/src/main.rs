use std::io;


//square
fn sqr () {
    println!("You've chosen Square!");
    let mut l = String::new();
    println!("\nInput length of square");
    io::stdin().read_line(&mut l).expect("not a valid input");
    let l:f32 = l.trim().parse().expect("not a valid number");

    let area = l * l;
    println!("Area of the square is {}", area);
}

//rectangle
fn rec () {
    println!("You've chosen Rectangle!");
    let mut l = String::new();
    println!("\nInput length of rectangle");
    io::stdin().read_line(&mut l).expect("not a valid input");
    let l:f32 = l.trim().parse().expect("not a valid number");

    let mut b = String::new();
    println!("Input breadth of rectangle");
    io::stdin().read_line(&mut b).expect("not a valid input");
    let b:f32 = b.trim().parse().expect("not a valid number");

    let area = l * b;
    println!("Area of the rectangle is {}", area);
}

//circle
fn cir () {
    println!("You've chosen Circle!");
    let mut r = String::new();
    println!("\nInput radius of circle");
    io::stdin().read_line(&mut r).expect("not a valid input");
    let r:f32 = r.trim().parse().expect("not a valid number");

    let area = 3.142 * r * r;
    println!("Area of the circle is {}", area);
}

//triangle
fn tri () {
    println!("You've chosen Triangle!");
    let mut b = String::new();
    println!("\nInput base of triangle");
    io::stdin().read_line(&mut b).expect("not a valid input");
    let b:f32 = b.trim().parse().expect("not a valid number");

    let mut h = String::new();
    println!("Input height of triangle");
    io::stdin().read_line(&mut h).expect("not a valid input");
    let h:f32 = h.trim().parse().expect("not a valid number");

    let area = b * h / 2.0;
    println!("Area of the triangle is {}", area);
}

//trapezium
fn trap() {
    println!("You've chosen Trapezium");
    let mut h = String::new();
    println!("\nInput Height");
    io::stdin().read_line(&mut h).expect("not a valid input");
    let h:f32 = h.trim().parse().expect("not a valid input");

    let mut a = String::new();
    println!("Input 1st parallel side (side a)");
    io::stdin().read_line(&mut a).expect("not a valid input");
    let a:f32 = a.trim().parse().expect("not a valid number");

    let mut b = String::new();
    println!("Input 2nd parallel side (side b)");
    io::stdin().read_line(&mut b).expect("not a valid input");
    let b:f32 = b.trim().parse().expect("not a valid number");

    let area = (a + b)/2.0 *h;
    
    println!("The area of the trapezium is: {}", area);
}

//rhombus
fn rom ()
 {
    println!("\nYou've chosen Rhombus!");
    println!("\ncalculate area using diagonals? (yes/no)");
    let mut rom = String::new();
    io::stdin().read_line(&mut rom).expect("invalid input :(");
    
    
    let choice = match rom.trim().to_lowercase().as_str() 
        {
        "yes" => true,
        "no" => false,
        _ => {
            println!("invalid input :(");
            false
            }
        }; 
    if choice 
    { //diagonals
        let mut d1 = String::new();
        println!("Input First diagonal");
        io::stdin().read_line(&mut d1).expect("Not a valid input");
        let d1:f32 = d1.trim().parse().expect("Not a valid number");

        let mut d2 = String::new();
        println!("Input Second diagonal");
        io::stdin().read_line(&mut d2).expect("Not a valid input");
        let d2:f32 = d2.trim().parse().expect("Not a valid number");

        let area = d1 * d2 / 2.0;
        
        println!("The area of the Rhombus is {}", area);
    }
    else 
    { //using base and height
        println!("\nUsing base and height; ");
        let mut b = String::new();
        println!("\nInput Base");
        io::stdin().read_line(&mut b).expect("Not a valid input");
        let b:f32 = b.trim().parse().expect("Not a valid number");

        let mut h = String::new();
        println!("Input Height");
        io::stdin().read_line(&mut h).expect("Not a valid input");
        let h:f32 = h.trim().parse().expect("Not a valid number");

        let area = b * h;
        
        println!("The area of rhombus is {}", area);
    }
 }

//parallelogram
fn par()
 {
    println!("You've chosen Parallelogram!");
 let mut b = String::new();
    println!("\nInput Base");
    io::stdin().read_line(&mut b).expect("Not a valid input");
    let b:f32 = b.trim().parse().expect("Not a valid number");

    let mut h = String::new();
    println!("Input Height");
    io::stdin().read_line(&mut h).expect("Not a valid input");
    let h:f32 = h.trim().parse().expect("Not a valid number");

    let area = b * h;
    
    println!("The area of parallelogram is {}", area); 
 }

//cube
fn cube () 
{
    println!("You've chosen Cube!");
    //surface area or volume?
    println!("\nSurface area or volume?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid input");

    let choice = match choice.trim().to_lowercase().as_str() 
    {
     "surface area" => true,
     "volume" => false,
     _=> {
        println!("not a valid option!");
        false
        }
    };

    if choice //surface area
    {
    let mut a = String::new();
    println!("Input Side of Square");
    io::stdin().read_line(&mut a).expect("Not a valid input");
    let a:f32 = a.trim().parse().expect("Not a valid number");

    let area = 6.0 * a * a;


    println!("The Surface Area of the cube is {}", area);   
    } 
    else 
        { //volume

    let mut l = String::new();
    println!("Input side of square");
    io::stdin().read_line(&mut l).expect("Not a valid input");
    let l:f32 = l.trim().parse().expect("Not a valid number");

    let vol = l * l * l;

    println!("The volume of the cube is {}", vol );
       }
}

fn cuboid () 
{
    println!("You've chosen Cuboid!");
    //surface area or volume?
    println!("\nSurface area or volume?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid input");

    let choice = match choice.trim().to_lowercase().as_str() 
    {
     "surface area" => true,
     "volume" => false,
     _=> {
        println!("not a valid option!");
        false
        }
    };

    let mut l = String::new();
    println!("Input length of rectangle");
    io::stdin().read_line(&mut l).expect("Not a valid input");
    let l:f32 = l.trim().parse().expect("Not a valid number");

    let mut b = String::new();
    println!("Input breadth of rectangle");
    io::stdin().read_line(&mut b).expect("Not a valid input");
    let b:f32 = b.trim().parse().expect("Not a valid number");

    let mut h = String::new();
    println!("Input height of rectangle");
    io::stdin().read_line(&mut h).expect("Not a valid input");
    let h:f32 = h.trim().parse().expect("Not a valid number");

    let area = 2.0 * ((l*b) + (b*h) + (h*l));
    let vol = l * b * h;
    
    if choice 
    {
        println!("The Surface area of the cuboid is {}", area)
    }
    else 
    {
        println!("The volume of the cuboid is {}", vol);
    }
}

//cylinder
fn cyl ()
{
    println!("You've chosen Cylinder!");
    let mut r = String::new();
    println!("\nInput radius");
    io::stdin().read_line(&mut r).expect("Not a valid input");
    let r:f32 = r.trim().parse().expect("Not a valid number");

    let mut h = String::new();
    println!("Input Height");
    io::stdin().read_line(&mut h).expect("Not a valid input");
    let h:f32 = h.trim().parse().expect("Not a valid number");

    let vol = 3.142 * r * r * h;

    println!("The volume of the cylinder is {} ",vol );
}

//sphere
fn sph () 
{
    println!("You've chosen Sphere!");
    let mut r = String::new();
    println!("\nInput radius of sphere");
    io::stdin().read_line(&mut r).expect("not a valid input");
    let r:f32 = r.trim().parse().expect("not a valid number");
    let vol = 4.0/3.0 * 3.142 * r * r * r;

    println!("The volume of the sphere is {}", vol);
}

//main code
fn main() 
{
    println!("\nWelcome to the shape calculator!
        \n What are we calculating today ?");
    println!("\nSquare
        \nRectangle
        \nTriangle
        \nCircle
        \nTrapezium
        \nRhombus
         \nParallelogram
         \nCube
         \nCuboid
         \nCylinder
         \nSphere");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid input");

match input.trim().to_lowercase().as_str() {
    "trapezium" => trap(),
    "rhombus" => rom(),
    "parallelogram" => par(),
    "cube" => cube(),
    "cylinder" => cyl(),
    "square" => sqr(),
    "rectangle" => rec(),
    "triangle" => tri(),
    "cuboid" => cuboid(),
    "circle" => cir(),
    "sphere" => sph(),
    
    _ => println!("Invalid selection. please select one of the shapes listed!"),
    }
}   

    


