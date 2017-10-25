use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 5, 255], OperandSize::Dword)
}

#[test]
fn vphsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 136834321, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 5, 52, 221, 17, 237, 39, 8], OperandSize::Dword)
}

#[test]
fn vphsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 5, 208], OperandSize::Qword)
}

#[test]
fn vphsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 1157215071, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 5, 144, 95, 179, 249, 68], OperandSize::Qword)
}

#[test]
fn vphsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 5, 212], OperandSize::Dword)
}

#[test]
fn vphsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 1231729924, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 5, 185, 4, 181, 106, 73], OperandSize::Dword)
}

#[test]
fn vphsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 5, 225], OperandSize::Qword)
}

#[test]
fn vphsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1351923669, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 5, 164, 143, 213, 183, 148, 80], OperandSize::Qword)
}

