use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 153, 249], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 153, 44, 67], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 153, 203], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 153, 16], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 220, 153, 238], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 153, 28, 191], OperandSize::Dword)
}

#[test]
fn vfmadd132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 245, 245, 153, 219], OperandSize::Qword)
}

#[test]
fn vfmadd132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 855672198, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 173, 131, 153, 172, 78, 134, 133, 0, 51], OperandSize::Qword)
}

