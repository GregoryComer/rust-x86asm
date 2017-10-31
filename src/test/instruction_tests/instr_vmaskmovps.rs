use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 1912212735, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 44, 175, 255, 12, 250, 113], OperandSize::Dword)
}

#[test]
fn vmaskmovps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1196542091, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 44, 36, 93, 139, 200, 81, 71], OperandSize::Qword)
}

#[test]
fn vmaskmovps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDI, 357343517, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 44, 175, 29, 161, 76, 21], OperandSize::Dword)
}

#[test]
fn vmaskmovps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDX, 512493407, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 44, 146, 95, 7, 140, 30], OperandSize::Qword)
}

#[test]
fn vmaskmovps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 46, 12, 135], OperandSize::Dword)
}

#[test]
fn vmaskmovps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 46, 49], OperandSize::Qword)
}

#[test]
fn vmaskmovps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectScaledDisplaced(ESI, Two, 2103640843, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 46, 60, 117, 11, 3, 99, 125], OperandSize::Dword)
}

#[test]
fn vmaskmovps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPS, operand1: Some(IndirectDisplaced(RCX, 490217540, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 46, 153, 68, 32, 56, 29], OperandSize::Qword)
}

