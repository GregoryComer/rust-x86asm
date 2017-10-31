use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 103, 230, 39], OperandSize::Dword)
}

#[test]
fn vfpclassss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K2)), operand2: Some(IndirectDisplaced(EBX, 573921846, Some(OperandSize::Dword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 11, 103, 147, 54, 90, 53, 34, 58], OperandSize::Dword)
}

#[test]
fn vfpclassss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM28)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 147, 125, 12, 103, 228, 87], OperandSize::Qword)
}

#[test]
fn vfpclassss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 9, 103, 44, 195, 23], OperandSize::Qword)
}

