#[derive(PartialEq)]
struct Bird<T> {
	x: T,
	y: T,
	z: T
}

trait Flying<T> {
	fn update_location(&mut self,T,T,T);
}

impl<T> Flying<T> for Bird<T> {
	fn update_location(&mut self, x: T, y: T, z: T) {
		self.x = x;
		self.y = y;
		self.z = z;
	}
}

#[test]
fn test1() {
	let a = Bird {
		x: 1f64,
		y: 100f64,
		z: 1.0
	};
	let mut b = Bird::<f64> {
		x: 1f64,
		y: 100f64,
		z: 1f64
	};
	assert!(a==b);
	b.update_location(1f64,2f64,3f64);
	assert!(a!=b);
}
