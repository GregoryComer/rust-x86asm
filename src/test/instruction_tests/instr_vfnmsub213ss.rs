use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 175, 226], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 930133161, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 175, 190, 169, 180, 112, 55], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 175, 208], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 175, 44, 251], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 191, 175, 202], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 175, 23], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 37, 211, 175, 250], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 117, 141, 175, 42], OperandSize::Qword)
}

