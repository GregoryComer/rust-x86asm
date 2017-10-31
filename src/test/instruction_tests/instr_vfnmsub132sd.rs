use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 159, 195], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1877621302, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 159, 140, 200, 54, 58, 234, 111], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 159, 213], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 2001123116, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 159, 172, 118, 44, 183, 70, 119], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 158, 159, 228], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 159, 12, 88], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 221, 185, 159, 199], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1710515832, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 189, 137, 159, 180, 135, 120, 102, 244, 101], OperandSize::Qword)
}

