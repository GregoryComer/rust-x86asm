use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 159, 204], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 159, 36, 154], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 159, 227], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 358389203, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 159, 140, 144, 211, 149, 92, 21], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 156, 159, 226], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1474497438, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 159, 162, 158, 11, 227, 87], OperandSize::Dword)
}

#[test]
fn vfnmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 133, 151, 159, 194], OperandSize::Qword)
}

#[test]
fn vfnmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 198926657, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 157, 134, 159, 172, 94, 65, 97, 219, 11], OperandSize::Qword)
}

