use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 221, 18], OperandSize::Dword)
}

#[test]
fn cmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1458372922, Some(OperandSize::Qword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 140, 90, 58, 1, 237, 86, 79], OperandSize::Dword)
}

#[test]
fn cmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 202, 37], OperandSize::Qword)
}

#[test]
fn cmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1571930383, Some(OperandSize::Qword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 60, 189, 15, 193, 177, 93, 3], OperandSize::Qword)
}

