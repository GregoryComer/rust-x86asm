use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 229], OperandSize::Dword)
}

#[test]
fn mulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1020103309, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 44, 141, 141, 138, 205, 60], OperandSize::Dword)
}

#[test]
fn mulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 195], OperandSize::Qword)
}

#[test]
fn mulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 566325665, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 140, 206, 161, 113, 193, 33], OperandSize::Qword)
}

