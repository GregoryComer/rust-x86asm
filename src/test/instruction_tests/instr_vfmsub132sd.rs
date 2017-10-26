use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 155, 239], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 486030115, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 155, 144, 35, 59, 248, 28], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 155, 249], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 155, 4, 182], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 158, 155, 246], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 155, 23], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 245, 183, 155, 207], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 165, 134, 155, 27], OperandSize::Qword)
}

