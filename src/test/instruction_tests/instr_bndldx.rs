use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndldx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDLDX, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledIndexed(EDX, ECX, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 26, 28, 10], OperandSize::Dword)
}

#[test]
fn bndldx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDLDX, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledIndexed(RSI, RSI, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 26, 4, 54], OperandSize::Qword)
}

