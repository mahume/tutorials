function countBs(string) {
    return countChar(string, 'B');
}
function countChar(string, char) {
    let count = 0;
    for (const character of string) {
        if (character === char) {
            count++;
        }
    }
    return count;
}
console.log(countBs('BBC'));