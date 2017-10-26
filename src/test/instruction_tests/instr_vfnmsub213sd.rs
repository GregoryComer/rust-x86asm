use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 250], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1815915086, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 175, 164, 183, 78, 170, 60, 108], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 194], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 927578521, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 172, 207, 153, 185, 73, 55], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 250, 175, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 175, 47], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 189, 243, 175, 218], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 149, 142, 175, 52, 121], OperandSize::Qword)
}

