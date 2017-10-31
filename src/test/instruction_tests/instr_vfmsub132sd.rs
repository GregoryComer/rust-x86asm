use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 155, 217], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1126555903, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 155, 140, 113, 255, 224, 37, 67], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 155, 214], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 1258012149, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 155, 139, 245, 189, 251, 74], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 188, 155, 228], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 34893891, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 155, 52, 205, 67, 112, 20, 2], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 189, 210, 155, 231], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 895755757, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 149, 134, 155, 4, 133, 237, 37, 100, 53], OperandSize::Qword)
}

