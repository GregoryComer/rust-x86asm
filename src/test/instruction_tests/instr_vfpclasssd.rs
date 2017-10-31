use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasssd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 15, 103, 201, 6], OperandSize::Dword)
}

#[test]
fn vfpclasssd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1369203812, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 9, 103, 180, 139, 100, 100, 156, 81, 98], OperandSize::Dword)
}

#[test]
fn vfpclasssd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 9, 103, 227, 104], OperandSize::Qword)
}

#[test]
fn vfpclasssd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 103, 12, 215, 113], OperandSize::Qword)
}

