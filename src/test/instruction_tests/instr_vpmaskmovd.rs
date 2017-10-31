use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 140, 44, 79], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RCX, 348391846, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 140, 161, 166, 9, 196, 20], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 746347481, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 140, 162, 217, 91, 124, 44], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1726502382, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 140, 20, 221, 238, 85, 232, 102], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 142, 30], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 142, 23], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectDisplaced(EAX, 1018466640, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 142, 128, 80, 145, 180, 60], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 585516331, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 142, 148, 122, 43, 69, 230, 34], OperandSize::Qword)
}

