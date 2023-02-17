const size = 12;

for (let y = 0; y < size; y++) {
  let line = "";
  for (let x = 0; x < size; x++) {
    if ((x + y) % 2 == 0) {
      line += " ";
    } else {
      line += "#";
    }
  }
  console.log(line);
}
