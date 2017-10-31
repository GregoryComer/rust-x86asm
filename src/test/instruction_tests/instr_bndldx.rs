use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndldx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDLDX, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledIndexed(EDI, EBX, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 26, 4, 31], OperandSize::Dword)
}

#[test]
fn bndldx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDLDX, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledIndexed(RCX, RBX, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 26, 20, 25], OperandSize::Qword)
}

