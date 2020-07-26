use std::ops::Range;

type ComplexF64 = (f64, f64);

fn main()
{
	let data = mandelbrot(-2.5..1.0, 0.03, -1.0..1.0, 0.03, 100.0, 100);

	for column in data {
		for cell in column {
			match cell {
				None => print!("██"),
				Some(value) if value < 4 => print!("  "),
				Some(value) if value < 5 => print!("░░"),
				Some(value) if value < 10 => print!("▒▒"),
				Some(value) if value < 100 => print!("▓▓"),
				Some(value) => print!("{:2}", value)
			}
		}
		println!();
	}
}

fn mandelbrot(x_range : Range<f64>, x_interval : f64, y_range : Range<f64>, y_interval : f64, max_value : f64, max_steps : u64) -> Vec<Vec<Option<u64>>>
{
	let x_iterations = ((x_range.end - x_range.start) / x_interval).ceil() as usize;
	let y_iterations = ((y_range.end - y_range.start) / y_interval).ceil() as usize;

	let mut result_data : Vec<Vec<Option<u64>>> = Vec::with_capacity(x_iterations);

	for y_step in 0..y_iterations {
		let mut row = Vec::with_capacity(x_iterations);
		for x_step in 0..x_iterations {
			let x = x_range.start + x_interval * x_step as f64;
			let y = y_range.start + y_interval * y_step as f64;

			row.push(stepcount((x, y), max_value, max_steps));
		}
		result_data.push(row);
	}

	result_data
}

fn stepcount(c : ComplexF64, max_value : f64, max_steps : u64) -> Option<u64>
{
	let mut z = (0.0, 0.0);

	for i in 0..max_steps {
		z = f(z, c);

		if (z.0 * z.0) + (z.1 * z.1) > max_value {
			return Some(i);
		}
	}

	None
}

fn f((zx, zy) : ComplexF64, (cx, cy) : ComplexF64) -> ComplexF64
{
	((zx * zx) - (zy * zy) + cx, 2.0 * zx * zy + cy)
}
