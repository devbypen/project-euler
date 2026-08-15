function result(target) {
	let a = 1;
	let b = 2;
	let sum = 0;

	while (a <= target) {
		if (a % 2 === 0) {
			sum = sum + a;
		}

		[a, b] = [b, a + b]
	}

	return sum;
}

console.time("Problem2")
console.log(result(4_000_000))
console.timeEnd("Problem2")

function resultTest(k) {
	let a = 1;
	let b = 2;
	let sum = 0;

	for (let i = 1; i <= k; i++) {
		if (a % 2 === 0) {
			sum = sum + a;
		} 

		[a, b] = [b, a + b]
	}

	return sum
}

console.log(resultTest(10))
