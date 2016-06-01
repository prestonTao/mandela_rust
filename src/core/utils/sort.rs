

pub trait Interface {
	fn Len(&self) -> u64;                    // Len is the number of elements in the collection.
	fn Less(&self, i: u64, j: u64) -> bool;  // index i should sort before the element with index j.
	fn Swap(&mut self, i: u64, j: u64);          // Swap swaps the elements with indexes i and j.
}

pub fn Sort<T: Interface>(data: &mut T){
	// 	n := data.Len()
	// maxDepth := 0
	// for i := n; i > 0; i >>= 1 {
	// 	maxDepth++
	// }
	// maxDepth *= 2
	// quickSort(data, 0, n, maxDepth)
	let mut n = data.Len();
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
	// println!("---- {}", maxDepth);
	// let mut a = 0;
	quickSort(data, 0, n, maxDepth);
}

fn quickSort<T: Interface>(data: &mut T, mut a: u64, mut b: u64, mut maxDepth: u64) {
	loop{
		if b-a <= 7{
			break;
		}
		if maxDepth == 0{
			heapSort(data, a, b);
			return;
		}
		maxDepth -= 1;
		let (mut mlo, mut mhi) = doPivot(data, a, b);
		// Avoiding recursion on the larger subproblem guarantees
		// a stack depth of at most lg(b-a).
		if mlo-a < b-mhi{
			quickSort(data, a, mlo, maxDepth);
			a = mhi; // i.e., quickSort(data, mhi, b)
		}else{
			quickSort(data, mhi, b, maxDepth);
			b = mlo; // i.e., quickSort(data, a, mlo)
		}
	}
	if b-a > 1 {
		insertionSort(data, a, b);
	}
}

fn heapSort<T: Interface>(data: &mut T, a: u64, b: u64) {
	let mut first = a;
	let mut lo = 0;
	let mut hi = b - a;

	// Build heap with greatest element at top.
	let mut count = (hi - 1) / 2;
	for i in (0..count) {
		siftDown(data, count-i, hi, first);
	}
	// Pop elements, largest first, into end of data.
	count = hi - 1;
	for i in (0..count){
		data.Swap(first, first + 1);
		siftDown(data, lo, count-i, first);
	}
}


fn doPivot<T: Interface>(data: &mut T, lo: u64, hi: u64) -> (u64, u64) {
	let m = lo + (hi-lo)/2;  // Written like this to avoid integer overflow.
	if hi-lo > 40 {
		// Tukey's ``Ninther,'' median of three medians of three.
		let s = (hi - lo) / 8;
		medianOfThree(data, lo, lo+s, lo+2*s);
		medianOfThree(data, m, m-s, m+s);
		medianOfThree(data, hi-1, hi-1-s, hi-1-2*s);
	}
	medianOfThree(data, lo, m, hi-1);

	// Invariants are:
	//	data[lo] = pivot (set up by ChoosePivot)
	//	data[lo <= i < a] = pivot
	//	data[a <= i < b] < pivot
	//	data[b <= i < c] is unexamined
	//	data[c <= i < d] > pivot
	//	data[d <= i < hi] = pivot
	//
	// Once b meets c, can swap the "= pivot" sections
	// into the middle of the slice.
	let pivot = lo;
	let (mut a, mut b, mut c, mut d) = (lo+1, lo+1, hi, hi);
	loop {
		loop{
			if b >= c{
				break;
			}
			if data.Less(b, pivot){
				b += 1;
			}else if !data.Less(pivot, b){
				data.Swap(a, b);
				a += 1;
				b += 1;
			}else{
				break;
			}
		}
		loop{
			if b >= c{
				break;
			}
			if data.Less(pivot, c-1) { // data[c-1] > pivot
				c -= 1;
			} else if !data.Less(c-1, pivot) { // data[c-1] = pivot
				data.Swap(c-1, d-1);
				c -= 1;
				d -= 1;
			} else {
				break;
			}
		}
		if b >= c {
			break;
		}
		// data[b] > pivot; data[c-1] < pivot
		data.Swap(b, c-1);
		b += 1;
		c -= 1;
	}

	let mut n = min(b-a, a-lo);
	swapRange(data, lo, b-n, n);

	n = min(hi-d, d-c);
	swapRange(data, c, hi-n, n);

	return (lo + b - a, hi - (d - c));
}

// Insertion sort
fn insertionSort<T: Interface>(data: &mut T, mut a: u64, b: u64) {
	let mut i = a + 1;
	loop{
		if i >= b{
			break;
		}
		let mut j = i;
		loop{
			if !(j > a && data.Less(j, j-1)){
				break;
			}
			data.Swap(j, j-1);
			j -= 1;
		}
		i += 1;
	}
}


// siftDown implements the heap property on data[lo, hi).
// first is an offset into the array where the root of the heap lies.
fn siftDown<T: Interface>(data: &mut T, lo: u64, hi: u64, first: u64) {
	let mut root = lo;
	loop{
		let mut child = 2*root + 1;
		if child >= hi {
			break;
		}
		if child+1 < hi && data.Less(first+child, first+child+1) {
			child += 1;
		}
		if !data.Less(first+root, first+child) {
			return;
		}
		data.Swap(first+root, first+child);
		root = child;
	}
}

// medianOfThree moves the median of the three values data[a], data[b], data[c] into data[a].
fn medianOfThree<T: Interface>(data: &mut T, a: u64, b: u64, c: u64) {
	let (m0, m1, m2) = (b, a, c);
	// bubble sort on 3 elements
	if data.Less(m1, m0) {
		data.Swap(m1, m0);
	}
	if data.Less(m2, m1) {
		data.Swap(m2, m1);
	}
	if data.Less(m1, m0) {
		data.Swap(m1, m0);
	}
	// now data[m0] <= data[m1] <= data[m2]
}

fn min(a: u64, b: u64) -> u64 {
	if a < b {
		return a;
	}
	return b;
}

fn swapRange<T: Interface>(data: &mut T, a: u64, b: u64, n: u64) {
	for i in (0..n){
		data.Swap(a+i, b+i);
	}
}