const lodash = require('lodash');
const puzzleInput = require('../util/puzzle-input');

const parse = (input) => input.split(',').map((x) => parseInt(x, 10));

const computeInstruction = (sequence, instructionPointer) => {
  const instruction = sequence[instructionPointer];
  switch (instruction) {
    case 1: {
      // Add
      const aAddr = sequence[instructionPointer + 1];
      const bAddr = sequence[instructionPointer + 2];
      const resultAddr = sequence[instructionPointer + 3];

      sequence[resultAddr] = sequence[aAddr] + sequence[bAddr];

      return { type: 'continue', newPosition: instructionPointer + 4 };
    }
    case 2: {
      // Multiply
      const aAddr = sequence[instructionPointer + 1];
      const bAddr = sequence[instructionPointer + 2];
      const resultAddr = sequence[instructionPointer + 3];

      sequence[resultAddr] = sequence[aAddr] * sequence[bAddr];

      return { type: 'continue', newPosition: instructionPointer + 4 };
    }
    case 99: {
      return { type: 'halt' };
    }
    default: {
      throw new Error(
        `Unrecognized instruction ${instruction} at instruction pointer ${instructionPointer}`
      );
    }
  }
};

const compute = (sequence) => {
  let instructionPointer = 0;
  while (true) {
    const result = computeInstruction(sequence, instructionPointer);
    if (result.type === 'continue') {
      instructionPointer = result.newPosition;
    } else if (result.type === 'halt') {
      return sequence[0];
    } else {
      throw new Error(`Unrecognized program state: ${result.type}`);
    }
  }
};

const bruteForceAnswer = (sequence, nounAddr, verbAddr, desiredOutput) => {
  const candidates = lodash.flatMap(lodash.range(0, 100), (nounCandidate) =>
    lodash.range(0, 100).map((verbCandidate) => [nounCandidate, verbCandidate])
  );

  const result = lodash.find(candidates, ([nounCandidate, verbCandidate]) => {
    const candidateSequence = [...sequence];
    candidateSequence[nounAddr] = nounCandidate;
    candidateSequence[verbAddr] = verbCandidate;
    const result = compute(candidateSequence);
    return result === desiredOutput;
  });
  if (result) {
    return result;
  } else {
    throw new Error('No answer found');
  }
};

describe('day02', () => {
  const PUZZLE_INPUT = puzzleInput.stringForDay('02');

  test('part two', () => {
    const sequence = parse(PUZZLE_INPUT);
    const [noun, verb] = bruteForceAnswer(sequence, 1, 2, 19690720);
    const result = 100 * noun + verb;
    expect(result).toEqual(4925);
  });
});
