function fSum(k) {

	let sum = 0;

	for (let i=1; i <=k; i++) {
		if (fNumber(i) % 2 === 0) {
			sum = sum + fNumber(i)
		}
	}

	return sum
	
}

// const memo = {}
// function fNumber(k) {
// 	if (k <= 2) {
// 		return k;
// 	}
//
// 	if (memo[k] !== undefined) {
// 		return memo[k]
// 	}
//
// 	memo[k] = fNumber(k-1) + fNumber(k-2);
// 	return memo[k]
// }
//

function fNumber(k) {
	if (k <= 2) {
		return k;
	}

	let first = 1;
	let second = 2;

	for (let i = 3; i <= k; i++) {
		tmp = first + second
		first = second	
		second = tmp
	}

	return second
}

function findK(target) {
	let i = 1;
	while (fNumber(i) <= target) {
		i++
	}
	return i - 1;
}

console.time("Problem 2");

console.log(fSum(findK(4_000_000)));

console.timeEnd("Problem 2");

// console.log(fSum(10))
// console.log(findK(4_000_000))
