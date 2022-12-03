import * as fs from 'fs';

const scores = new Map<string, number>([
	['AZ', 3],
	['AX', 4],
	['AY', 8],
	['BX', 1],
	['BY', 5],
	['BZ', 9],
	['CX', 7],
	['CY', 2],
	['CZ', 6],
	['AA', 4],
	['BB', 5],
	['CC', 6],
]);

const win = new Map<string, string>([
	['A', 'Y'],
	['B', 'Z'],
	['C', 'X'],
]);

const lose = new Map<string, string>([
	['A', 'Z'],
	['B', 'X'],
	['C', 'Y'],
]);

(() => {
	let sum = 0;
	for (const [p1, p2] of fs
		.readFileSync(`${__dirname}/input.txt`, 'utf8')
		.split('\n')
		.map((line) => line.replace(/ /g, ''))
		.map((line) => line.split(''))) {
		switch (p2) {
			case 'Z':
				sum += scores.get(`${p1}${win.get(p1)}`)!;
				break;
			case 'Y':
				sum += scores.get(`${p1}${p1}`)!;
				break;
			case 'X':
				sum += scores.get(`${p1}${lose.get(p1)}`)!;
				break;
		}
	}
	console.log(sum);
})();
