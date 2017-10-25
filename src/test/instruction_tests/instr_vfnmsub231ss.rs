use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 191, 222], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1076595639, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 191, 185, 183, 139, 43, 64], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 191, 235], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 241401114, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 191, 172, 200, 26, 125, 99, 14], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 187, 191, 237], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 725618088, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 191, 140, 210, 168, 13, 64, 43], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 101, 244, 191, 202], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RDI, 1970050692, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 61, 143, 191, 183, 132, 150, 108, 117], OperandSize::Qword)
}

