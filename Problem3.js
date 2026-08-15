function result(number) {
	let array = [];
	// square root of number
	for (let i = 2; i * i <= number; i++) {
		while (number  % i === 0) {
			array.push(i)
			number = number / i
		}
	}

	if (number > 1) {
		array.push(number)
	}

	return array
}

console.log(result(600851475143))
