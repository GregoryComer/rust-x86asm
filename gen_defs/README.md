# Instruction Generation

Instruction defitions are programatically generated from the Intel x86 reference manual. The instruction generation code reads from a csv representation extracted from the Intel reference manual. This csv is bundled with this repository. For information on this csv, see https://github.com/GregoryComer/x86-csv.

Once read, each row in the csv is converted into an InstructionDefinition and written to stdout. When run with --tests, tests are also output to the test directory.

## Running
To run the instruction & test generator, run the shell script make_all.sh. This requires GNU AS to be installed for test generation. To copy the output to the main crate, run deploy.sh.
