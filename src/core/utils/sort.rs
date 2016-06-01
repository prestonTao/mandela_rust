

pub trait Interface {
	fn Len(&self) -> usize;                    // Len is the number of elements in the collection.
	fn Less(&self, i: usize, j: usize) -> bool;  // index i should sort before the element with index j.
	fn Swap(&self, i: usize, j: usize);          // Swap swaps the elements with indexes i and j.
}

pub fn Sort<T: Interface>(data: &mut T){
	let n = data.Len();
	let mut maxDepth = 0;
	let mut i = n;
	loop{
		if i <= 0{
			break;
		}
		maxDepth += 1;
		i >>= 1;
	}
	// for i = n; i > 0; i >>= 1 {
	// 	maxDepth += 1;
	// }
	maxDepth *= 2;
	println!("{}", maxDepth);
}