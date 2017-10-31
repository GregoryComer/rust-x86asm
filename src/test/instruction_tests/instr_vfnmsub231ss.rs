use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 191, 250], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 191, 52, 65], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 191, 199], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 191, 60, 72], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 185, 191, 197], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1333738937, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 191, 151, 185, 61, 127, 79], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 45, 246, 191, 204], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RCX, 2025994989, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 37, 132, 191, 169, 237, 58, 194, 120], OperandSize::Qword)
}

