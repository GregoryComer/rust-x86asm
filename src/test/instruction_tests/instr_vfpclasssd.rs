use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasssd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 10, 103, 219, 65], OperandSize::Dword)
}

#[test]
fn vfpclasssd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 584779753, Some(OperandSize::Qword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 10, 103, 156, 142, 233, 7, 219, 34, 74], OperandSize::Dword)
}

#[test]
fn vfpclasssd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 13, 103, 209, 68], OperandSize::Qword)
}

#[test]
fn vfpclasssd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K5)), operand2: Some(IndirectDisplaced(RBX, 145262406, Some(OperandSize::Qword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 11, 103, 171, 70, 135, 168, 8, 125], OperandSize::Qword)
}

