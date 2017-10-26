use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasssd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 103, 209, 64], OperandSize::Dword)
}

#[test]
fn vfpclasssd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 400713374, Some(OperandSize::Qword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 9, 103, 156, 113, 158, 102, 226, 23, 105], OperandSize::Dword)
}

#[test]
fn vfpclasssd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM26)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 253, 10, 103, 210, 24], OperandSize::Qword)
}

#[test]
fn vfpclasssd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1186181023, Some(OperandSize::Qword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 12, 103, 44, 69, 159, 175, 179, 70, 90], OperandSize::Qword)
}

