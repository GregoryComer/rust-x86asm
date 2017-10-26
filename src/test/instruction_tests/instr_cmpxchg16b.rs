use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg16b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG16B, operand1: Some(IndirectDisplaced(RAX, 1028599597, Some(OperandSize::Xmmword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 136, 45, 47, 79, 61], OperandSize::Qword)
}

