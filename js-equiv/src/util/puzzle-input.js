const fs = require('fs');

exports.stringForDay = (day) =>
  fs.readFileSync(`src/days/day${day}-input.txt`, { encoding: 'utf-8' });
