use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 171, 229], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 245130727, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 171, 182, 231, 101, 156, 14], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 171, 219], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 171, 12, 130], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 218, 171, 236], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1838273658, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 139, 171, 147, 122, 212, 145, 109], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 253, 150, 171, 224], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RDX, 2108151108, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 189, 135, 171, 130, 68, 213, 167, 125], OperandSize::Qword)
}

