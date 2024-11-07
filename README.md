# IEEE Rust Workshop -- Hands-On Portion

Welcome!! In this portion, we'll be walking you through writing your own _very first_ Rust project! 

We'll be simulating a [Lorentz Attractor](https://en.wikipedia.org/wiki/Lorenz_system), a _beautiful_ mathematical system, that originally emerged as a simplified model of the atmosphere, and has become a standard illustration of chaotic behavior. 

We'll simulate the equations for the system, and then visualize them with plot.ly -- you can see our final code in src/main.rs. 

## Step 1: Getting started

Open up your terminal, create a new Rust project, with `cargo new lorenz`, and navigate into the project directory with `cd lorenz`.

Install plot.ly, with `cargo add plotly` (check that a line was added to your Cargo.toml file). 

Copy the [`skeleton.rs`](src/skeleton.rs) file in this repository (in the `src` directory) into your `src/main.rs` file, and run `cargo build`. 

Our final code is in the `src/main.rs` file -- feel free to look at it if you get stuck, or ask us for help.

## Step 2: Simulating the System
Look at the `lorenz_attractor` function -- it turns a `Vec<[f64; 3]>`, that is, a list ("vector") of 3-element arrays of floats. These are each of the points in the Lorenz system.  

Delete the `todo!()` inside of the argument to `map`, and replace it with code that does this following (the argument to `map` is a _closure_ that will be called `STEPS` times):

* View current value of `p` as three floats `x, y, z`. 
* The Lorenz system is defined by these equations, so compute each of `dx`, `dy`, and `dz`.
    * `dx = SIGMA * (y - x)`
    * `dy = x * (RHO - z) - y`
    * `dz = xy - BETA * z`
* Add `dx * DT` to `p[0]`, `dy * DT` to `p[1]`, and `dz * DT` to `p[2]`
* Return `p`.

This is computing each step of the Lorenz system, updating the position `p` at each step, and collecting all the `p`s into a vector, returning it.

## Step 3: Plotting

We're using `plotly` -- like most Rust crates (Rust's word for "libraries"), [plot.ly's documentation](https://docs.rs/plotly/latest/plotly/index.html) is wonderful. 

We've taken the `Vec<[f64; 3]>` that `lorenz_attractor` returns, and separated it out into `3` `Vec<f64>`s, `xs`, `ys`, and `zs` for you (feel free to ask if you want to know how this line works!). 

Plotly has a `Scatter3D` object that we want to use -- use `Scatter3D::new`, and pass in `xs`, `ys`, and `zs` as arguments. On this object, call `.mode(Mode::Lines)` (to not plot points) (and feel free to set the `name`, the line style/color/width, etc.) (Check the documentation to make sure the types make sense!). Note that the `.mode` method consumes and replaces the `Scatter3D` object. 

Then, create a `Plot` object (`Plot::new`), and call `.add_trace` on it, passing in the `Scatter3D` object. 

Finally, call `.show` on the `Plot` object, and you should see a beautiful Lorenz Attractor!


