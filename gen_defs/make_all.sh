#!/bin/bash
rm instruction_defs.rs

cargo run -- --tests \
    | sed 's/^/\t/g' \
    | sed 's/encoding: /encoding: OperandEncoding::/g' \
    | sed 's/access: /access: OperandAccess::/g' \
    | sed 's/operand_behavior: Some(/operand_behavior: Some(VexOperandBehavior::/g' \
    | sed 's/type: /type: OperandType::/g' \
    | sed 's/composite_prefix: Some(/composite_prefix: Some(CompositePrefix::/g' \
    | sed 's/InstructionDefinition:/InstructionDefinition/g' \
    | sed 's/Fixed(Reg(/Fixed(FixedOperand::Reg(Reg::/g' \
    | sed 's/Fixed(Constant(/Fixed(FixedOperand::Constant(/g' \
    | sed 's/mnemonic: "\(.*\)"/mnemonic: Mnemonic::\1/g' \
    | sed 's/OperandType::Set(/OperandType::Set(\&/g' \
    > temp

DEF_COUNT=`grep -o InstructionDefinition temp | wc -l`

echo "use ::{Mnemonic, OperandSize, Reg};
use ::RegType::*;
use ::instruction_def::*;
use ::instruction_def::OperandType::*;
use ::instruction_def::OperandSizePrefixBehavior::*;
" >> instruction_defs.rs

echo "pub static INSTR_DEFS: [InstructionDefinition; $DEF_COUNT] = [" >> instruction_defs.rs
cat temp >> instruction_defs.rs
echo "];" >> instruction_defs.rs
rm temp

# Generate mnemonics
mnemonics=$(grep -o Mnemonic::[A-Za-z0-9]* instruction_defs.rs | sed 's/Mnemonic:://g' | sort | uniq)
echo '#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]' > mnemonic.rs
echo 'pub enum Mnemonic {' >> mnemonic.rs
for mnem in $mnemonics; do
    echo "    $mnem," >> mnemonic.rs
done
echo '}' >> mnemonic.rs
echo >> mnemonic.rs
echo 'impl Mnemonic {' >> mnemonic.rs
echo '    pub fn parse(val: &str) -> Result<Mnemonic, ()> {' >> mnemonic.rs
echo '        match val {' >> mnemonic.rs
for mnem in $mnemonics; do
    echo "            \"$mnem\" => Ok(Mnemonic::$mnem)," >> mnemonic.rs
done
echo "            _ => Err(())" >> mnemonic.rs
echo "        }" >> mnemonic.rs
echo "    }" >> mnemonic.rs
echo "}" >> mnemonic.rs

# Bundle tests
rm -rf instruction_tests
mkdir instruction_tests
for test_file in test/*; do
    MNEMONIC=`basename $test_file`
    TEST_OUT="instruction_tests/instr_$MNEMONIC.rs"

    echo -e "use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};" > $TEST_OUT
    echo -e "use ::RegType::*;" >> $TEST_OUT
    echo -e "use ::instruction_def::*;" >> $TEST_OUT
    echo -e "use ::Operand::*;" >> $TEST_OUT
    echo -e "use ::Reg::*;" >> $TEST_OUT
    echo -e "use ::RegScale::*;" >> $TEST_OUT
    echo -e "use ::test::run_test;" >> $TEST_OUT
    echo -e "" >> $TEST_OUT

    cat $test_file \
        | sed 's/rounding_mode: Some(/rounding_mode: Some(RoundingMode::/g' \
        | sed 's/merge_mode: Some(/merge_mode: Some(MergeMode::/g' \
        | sed 's/broadcast: Some(/broadcast: Some(BroadcastMode::/g' \
        | sed 's/mask: Some(/mask: Some(MaskReg::/g' \
        | sed 's/mnemonic: "\(.*\)"/mnemonic: Mnemonic::\1/g' \
        >> $TEST_OUT

    echo -e "mod instr_$MNEMONIC;" >> instruction_tests/mod.rs
done
