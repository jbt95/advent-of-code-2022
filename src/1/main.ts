import * as fs from 'fs';

(() => {
	const parts = fs
		.readFileSync(`${__dirname}/input.txt`, 'utf8')
		.split('\n')
		.map((line) => line.replace(/ /g, ''));
	const result: number[] = [];
	let sum = 0;
	for (let i = 0; i < parts.length; i++) {
		if (parts[i + 1] === '') {
			result.push(sum + Number(parts[i]));
			sum = 0;
			continue;
		}
		sum += Number(parts[i]);
	}
	console.log(result.sort((a, b) => b - a)[0]);
})();
