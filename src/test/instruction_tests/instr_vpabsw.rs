use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 226], OperandSize::Dword)
}

#[test]
fn vpabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1785450663, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 60, 69, 167, 208, 107, 106], OperandSize::Dword)
}

#[test]
fn vpabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 251], OperandSize::Qword)
}

#[test]
fn vpabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 96220447, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 44, 141, 31, 53, 188, 5], OperandSize::Qword)
}

#[test]
fn vpabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 254], OperandSize::Dword)
}

#[test]
fn vpabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 740933150, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 60, 221, 30, 190, 41, 44], OperandSize::Dword)
}

#[test]
fn vpabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 227], OperandSize::Qword)
}

#[test]
fn vpabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 31], OperandSize::Qword)
}

#[test]
fn vpabsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 29, 234], OperandSize::Dword)
}

#[test]
fn vpabsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDX, 1115681090, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 29, 146, 66, 241, 127, 66], OperandSize::Dword)
}

#[test]
fn vpabsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 29, 213], OperandSize::Qword)
}

#[test]
fn vpabsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM8)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 141, 29, 1], OperandSize::Qword)
}

