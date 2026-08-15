function result(target) {
	let a = 2;
	let b = 8;
	let sum = 0;

	while (a <= target) {
		sum = sum + a;
		[a, b] = [b, 4 * b + a]
	}

	return sum;
}

console.time("Problem2")
console.log(result(4_000_000));
console.timeEnd("Problem2")
