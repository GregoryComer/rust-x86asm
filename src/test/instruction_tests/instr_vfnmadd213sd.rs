use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 173, 216], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 173, 12, 75], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 173, 234], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 736544822, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 173, 155, 54, 200, 230, 43], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 154, 173, 242], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 675562073, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 173, 172, 178, 89, 66, 68, 40], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 237, 255, 173, 239], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 141, 139, 173, 1], OperandSize::Qword)
}

