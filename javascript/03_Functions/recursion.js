function isEven(int) {
    if (int === 0) {
        return true;
    } else if (int === 1) {
        return false;
    } else {
        return isEven(int - 2);
    }
}
console.log(isEven(50));