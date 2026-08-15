// n + 2n + 3n + 4n + 5n + ...+ kn
// n (1 + 2 + 3 + 4 + 5 + ... + k)
// n (1 + k)*k/2

function sumMultiple(n, limit) {
	const k = Math.floor((limit-1) / n);

	return n*(1+k)*k/2;
}

function result(limit) {
	return (
		sumMultiple(3, limit) +
		sumMultiple(5, limit) -
		sumMultiple(15, limit)
	);
}

console.log(result(1000));
