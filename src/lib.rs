// Macro to handle fixme messages
// FIXME: Implement export in log directory based on system used
// FIXME: Allow end-user to customize the message
// FIXME: Allow end-user to mute the messages through configuration
// FIXME: Contribute in log to outsource maintainance on them
#[macro_export]
macro_rules! fixme {
	($msg:expr) => ( println!("FIXME: {}", $msg);)
}

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

//#[cfg(test)]
//if fibonacci(5) != "38" {
//	die(1; "Test for fibonacci(5) did not return 38, returned '{}'", fibonacci(5));
//};