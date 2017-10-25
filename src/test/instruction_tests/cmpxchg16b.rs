use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmpxchg16b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG16B, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 2129208817, Some(OperandSize::Xmmword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 140, 207, 241, 37, 233, 126], OperandSize::Qword)
}

