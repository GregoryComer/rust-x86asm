use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 171, 250], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 1969518316, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 171, 129, 236, 118, 100, 117], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 171, 248], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 1797783066, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 171, 178, 26, 254, 39, 107], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 187, 171, 210], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 488914377, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 171, 44, 221, 201, 61, 36, 29], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 53, 217, 171, 206], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2015251109, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 93, 137, 171, 44, 93, 165, 74, 30, 120], OperandSize::Qword)
}

