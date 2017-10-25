use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 103, 202, 87], OperandSize::Dword)
}

#[test]
fn vfpclassss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 12, 103, 44, 198, 97], OperandSize::Dword)
}

#[test]
fn vfpclassss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM10)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 125, 12, 103, 218, 14], OperandSize::Qword)
}

#[test]
fn vfpclassss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 77612983, Some(OperandSize::Dword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 10, 103, 172, 152, 183, 71, 160, 4, 118], OperandSize::Qword)
}

