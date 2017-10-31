use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 93, 195], OperandSize::Dword)
}

#[test]
fn vminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 93, 12, 246], OperandSize::Dword)
}

#[test]
fn vminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 93, 227], OperandSize::Qword)
}

#[test]
fn vminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 93, 20, 75], OperandSize::Qword)
}

#[test]
fn vminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 159, 93, 236], OperandSize::Dword)
}

#[test]
fn vminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 647280727, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 207, 143, 93, 140, 193, 87, 184, 148, 38], OperandSize::Dword)
}

#[test]
fn vminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 175, 156, 93, 207], OperandSize::Qword)
}

#[test]
fn vminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RCX, 687927536, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 191, 137, 93, 129, 240, 240, 0, 41], OperandSize::Qword)
}

