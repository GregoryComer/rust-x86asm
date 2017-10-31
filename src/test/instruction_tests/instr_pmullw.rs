use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 207], OperandSize::Dword)
}

#[test]
fn pmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDX, 7787480, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 186, 216, 211, 118, 0], OperandSize::Dword)
}

#[test]
fn pmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 249], OperandSize::Qword)
}

#[test]
fn pmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1904176634, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 180, 126, 250, 109, 127, 113], OperandSize::Qword)
}

#[test]
fn pmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 251], OperandSize::Dword)
}

#[test]
fn pmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 1024500135, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 154, 167, 161, 16, 61], OperandSize::Dword)
}

#[test]
fn pmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 252], OperandSize::Qword)
}

#[test]
fn pmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1882429516, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 52, 93, 76, 152, 51, 112], OperandSize::Qword)
}

