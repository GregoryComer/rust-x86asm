rm instruction_defs.rs 2> /dev/null
rm instruction_defs_raw.rs 2> /dev/null
rm mnemonic.rs 2> /dev/null

# Dump raw entries & add enum specifiers
cargo run | sed 's/^/    /
s/mnemonic: /mnemonic: Mnemonic::/g
s/proc_start: Some(/proc_start: Some(ProcessorLevel::/g
s/proc_end: Some(/proc_end: Some(ProcessorLevel::/g
s/ mode: / mode: Mode::/g
s/ring_level: /ring_level: RingLevel::/g
s/addressing_mode: /addressing_mode: OperandAddressingMode::/g
s/operand_type: /operand_type: OperandType::/g
s/fixed_operand: Some(/fixed_operand: Some(FixedOperand::/g
s/op_size_64_behavior: /op_size_64_behavior: OpSize64Behavior::/g' |
# Special handling for CBW/CWD/INSW/LODSW/CMPSW/OUTSW (force operand size prefix)
sed 's/\(CBW.*force_op_size_prefix:\) false/\1 true/
s/\(CWD,.*force_op_size_prefix:\) false/\1 true/
s/\([^M]INSW,.*force_op_size_prefix:\) false/\1 true/
s/\(LODSW,.*force_op_size_prefix:\) false/\1 true/
s/\(CMPSW,.*force_op_size_prefix:\) false/\1 true/
s/\(OUTSW,.*force_op_size_prefix:\) false/\1 true/
s/\(MOVSW,.*force_op_size_prefix:\) false/\1 true/' |
# Special handling for JCXE (force address size prefix)
sed 's/\(JCXZ.*force_addr_size_prefix:\) false/\1 true/' |
# Special handling for *FENCE (set secondary opcode, remove opcode extension, maybe could do this better)
sed 's/\(LFENCE.*secondary_opcode:\) None\(.*opcode_ext:\) Some(.)/\1 Some(232) \2 None/
s/\(MFENCE.*secondary_opcode:\) None\(.*opcode_ext:\) Some(.)/\1 Some(240) \2 None/' |
# Special handling for CVTDQ2PD (change operand type to XMMorMem64)
sed 's/\(CVTDQ2PD.*\)operand_type: OperandType::DQ/\1operand_type: OperandType::XMMorMem64/' |
# Remove def for DPPD (seems wrong, replaced in extra defs), ADX
sed '/DPPD/d
/Mnemonic::ADX/d' |
# Change operand type to XMM for MOVSLDUP
sed 's/\(MOVS[LH]DUP.*\)OperandType::Q\(.*\)OperandType::Q/\1OperandType::XMM\2OperandType::XMM/' |
# Allow SMM instructions in any mode. TODO?
sed 's/SystemManagement/Real/' > instruction_defs_raw.rs

# Add extra defs
cat extra_defs.rs >> instruction_defs_raw.rs
./gen_avx_defs.sh

DEF_COUNT=`grep -o InstructionDefinition[^F] instruction_defs_raw.rs | wc -l`
echo "Generated $DEF_COUNT defs."

# Add initial lines
echo "use ::{ Mnemonic, Mode, ProcessorLevel, BroadcastMode };
use ::instruction_def::{InstructionDefinition, InstructionDefinitionFlags, OperandAddressingMode, OperandType, OperandDescription, RingLevel, FixedOperand, OpSize64Behavior };

pub static INSTR_DEFS: [InstructionDefinition; $DEF_COUNT] = [" > instruction_defs.rs

# Append entries to file
cat instruction_defs_raw.rs >> instruction_defs.rs

# Close array
echo "];" >> instruction_defs.rs

# Generate mnemonics
mnemonics=$(grep -o Mnemonic::[A-Z0-9]* instruction_defs.rs | sed 's/Mnemonic:://g' | sort | uniq)
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
