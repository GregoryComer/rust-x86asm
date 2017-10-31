use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1707257251, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 140, 148, 255, 163, 173, 194, 101], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 698184944, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 140, 151, 240, 116, 157, 41], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 62982519, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 140, 156, 246, 119, 9, 193, 3], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 140, 28, 153], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 142, 26], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 214029873, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 142, 140, 248, 49, 214, 193, 12], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1992091565, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 142, 132, 159, 173, 231, 188, 118], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 142, 44, 128], OperandSize::Qword)
}

