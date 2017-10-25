use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasssd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 13, 103, 204, 14], OperandSize::Dword)
}

#[test]
fn vfpclasssd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1294817187, Some(OperandSize::Qword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 103, 12, 85, 163, 87, 45, 77, 14], OperandSize::Dword)
}

#[test]
fn vfpclasssd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM11)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 253, 14, 103, 251, 90], OperandSize::Qword)
}

#[test]
fn vfpclasssd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 253534099, Some(OperandSize::Qword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 103, 12, 189, 147, 159, 28, 15, 25], OperandSize::Qword)
}

