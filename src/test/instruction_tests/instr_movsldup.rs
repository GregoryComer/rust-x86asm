use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 215], OperandSize::Dword)
}

#[test]
fn movsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 36, 177], OperandSize::Dword)
}

#[test]
fn movsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 201], OperandSize::Qword)
}

#[test]
fn movsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1131044442, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 52, 125, 90, 94, 106, 67], OperandSize::Qword)
}

