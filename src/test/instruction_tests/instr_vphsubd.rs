use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 6, 203], OperandSize::Dword)
}

#[test]
fn vphsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 121464495, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 6, 44, 189, 175, 102, 61, 7], OperandSize::Dword)
}

#[test]
fn vphsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 6, 237], OperandSize::Qword)
}

#[test]
fn vphsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 6, 4, 123], OperandSize::Qword)
}

#[test]
fn vphsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 6, 244], OperandSize::Dword)
}

#[test]
fn vphsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 6, 28, 243], OperandSize::Dword)
}

#[test]
fn vphsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 6, 232], OperandSize::Qword)
}

#[test]
fn vphsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1331044196, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 6, 132, 80, 100, 31, 86, 79], OperandSize::Qword)
}

