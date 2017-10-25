use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 220], OperandSize::Dword)
}

#[test]
fn rsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 63], OperandSize::Dword)
}

#[test]
fn rsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 213], OperandSize::Qword)
}

#[test]
fn rsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDX, 1844738751, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 178, 191, 122, 244, 109], OperandSize::Qword)
}

