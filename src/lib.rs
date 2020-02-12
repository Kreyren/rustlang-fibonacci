// NOTICE: Has to be inlined, because inlining does not happend across crate boundaries unless inline is set (difference of +-70% of efficiency)
#[inline]
pub fn fibonacci() -> impl Iterator<Item = u128> {
	let mut state = [1, 1];
		std::iter::from_fn(move || {
			state.rotate_left(1);
			let next = state[0] + state[1];
			Some(std::mem::replace(&mut state[1], next))
	})
}