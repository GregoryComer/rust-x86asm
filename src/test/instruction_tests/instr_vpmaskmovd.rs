use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 776047173, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 140, 160, 69, 138, 65, 46], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 140, 52, 80], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1393339528, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 140, 145, 136, 172, 12, 83], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RBX, 1075717139, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 140, 163, 19, 36, 30, 64], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 142, 26], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 142, 4, 194], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 142, 25], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 856145445, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 142, 156, 88, 37, 190, 7, 51], OperandSize::Qword)
}

