use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 191, 194], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 191, 26], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 191, 220], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RCX, 615798216, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 191, 169, 200, 85, 180, 36], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 186, 191, 252], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 191, 60, 211], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 37, 214, 191, 234], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1041936447, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 53, 129, 191, 140, 255, 63, 176, 26, 62], OperandSize::Qword)
}

