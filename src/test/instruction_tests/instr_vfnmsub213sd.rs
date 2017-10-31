use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 175, 237], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1332860417, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 163, 1, 214, 113, 79], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 175, 202], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 56], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 221, 175, 232], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 139, 175, 51], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 133, 217, 175, 194], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RAX, 80088984, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 157, 129, 175, 160, 152, 15, 198, 4], OperandSize::Qword)
}

