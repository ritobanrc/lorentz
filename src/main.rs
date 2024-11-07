use plotly::common::Mode;
use plotly::layout::{Layout, Margin};
use plotly::traces::Scatter3D;
use plotly::Plot;

// Constants for Lorenz attractor
const SIGMA: f64 = 10.0;
const RHO: f64 = 28.0;
const BETA: f64 = 8.0 / 3.0;
const DT: f64 = 0.01; // Time step
const STEPS: usize = 10_000; // Number of iterations

/// Function to compute the Lorenz attractor points
fn lorenz_attractor() -> Vec<[f64; 3]> {
    let mut p = [1.; 3];

    (0..STEPS)
        .map(|_| {
            // Calculate derivatives using the Lorenz equations
            let [x, y, z] = p;
            let dx = SIGMA * (y - x);
            let dy = x * (RHO - z) - y;
            let dz = x * y - BETA * z;

            // Update positions
            p = [p[0] + dx * DT, p[1] + dy * DT, p[2] + dz * DT];

            p
        })
        .collect()
}

/// Function to plot the Lorenz attractor
fn plot_lorenz(trace: Vec<[f64; 3]>) {
    let (xs, (ys, zs)) = trace.into_iter().map(|[x, y, z]| (x, (y, z))).unzip();

    let trace = Scatter3D::new(xs, ys, zs)
        .mode(Mode::Lines)
        .name("Lorenz Attractor")
        .line(plotly::common::Line::new().width(2.0).color("blue"));
    let layout = Layout::new()
        .width(1920) // Adjust width if necessary
        .height(1000) // Adjust height if necessary
        .margin(Margin::new().left(0).right(0).top(0).bottom(0)); // Remove margins

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.show();
}

fn main() {
    // Generate Lorenz attractor data
    let trace = lorenz_attractor();

    // Plot the Lorenz attractor
    plot_lorenz(trace);
}
