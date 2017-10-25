use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 175, 225], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1508557997, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 175, 188, 155, 173, 196, 234, 89], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 175, 223], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 886651422, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 175, 154, 30, 58, 217, 52], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 189, 175, 244], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 175, 54], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 149, 177, 175, 212], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1676355206, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 189, 140, 175, 140, 187, 134, 38, 235, 99], OperandSize::Qword)
}

