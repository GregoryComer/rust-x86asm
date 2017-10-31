use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 247], OperandSize::Dword)
}

#[test]
fn subps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 14], OperandSize::Dword)
}

#[test]
fn subps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 249], OperandSize::Qword)
}

#[test]
fn subps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1371585034, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 60, 85, 10, 186, 192, 81], OperandSize::Qword)
}

