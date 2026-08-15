const sn = 309000;
let sum = 0;


for(let i=1; i < 309000; i++ ) {
	if (i % 2 === 1) {
		sum = sum + i*i;
	}
}

console.log(sum);
