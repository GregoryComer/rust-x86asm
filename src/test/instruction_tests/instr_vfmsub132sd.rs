use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 155, 194], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 155, 62], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 155, 200], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 155, 9], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 156, 155, 221], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 878480296, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 155, 164, 186, 168, 139, 92, 52], OperandSize::Dword)
}

#[test]
fn vfmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 237, 177, 155, 225], OperandSize::Qword)
}

#[test]
fn vfmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 1316841025, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 140, 155, 161, 65, 102, 125, 78], OperandSize::Qword)
}

