use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 175, 249], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 1420896054, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 175, 128, 54, 39, 177, 84], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 175, 248], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 175, 60, 185], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 251, 175, 219], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 867314036, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 138, 175, 132, 199, 116, 41, 178, 51], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 101, 249, 175, 232], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RAX, 113123105, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 5, 143, 175, 168, 33, 31, 190, 6], OperandSize::Qword)
}

