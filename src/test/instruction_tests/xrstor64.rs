use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xrstor64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR64, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 2018336743, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 174, 172, 248, 231, 95, 77, 120], OperandSize::Qword)
}

