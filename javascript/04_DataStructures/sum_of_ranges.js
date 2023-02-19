function range(start, end, step = 1) {
    let numbers = [];

    for (let i = start; i <= end; i += step) {
        numbers.push(i);
    }

    return numbers;
}
function sum(numbers) {
    let total = 0;
    for (let number of numbers) {
        total += number;
    }
    return total;
}
console.log(sum(range(1, 10)));