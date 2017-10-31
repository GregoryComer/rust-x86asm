use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 15, 103, 237, 12], OperandSize::Dword)
}

#[test]
fn vfpclassss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K3)), operand2: Some(IndirectDisplaced(ESI, 572479533, Some(OperandSize::Dword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 103, 158, 45, 88, 31, 34, 45], OperandSize::Dword)
}

#[test]
fn vfpclassss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM11)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 125, 15, 103, 203, 77], OperandSize::Qword)
}

#[test]
fn vfpclassss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K4)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 103, 32, 39], OperandSize::Qword)
}

