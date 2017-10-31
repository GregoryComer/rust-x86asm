use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 207], OperandSize::Dword)
}

#[test]
fn pmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(EDX, 1800355419, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 178, 91, 62, 79, 107], OperandSize::Dword)
}

#[test]
fn pmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 216], OperandSize::Qword)
}

#[test]
fn pmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1177398598, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 36, 197, 70, 173, 45, 70], OperandSize::Qword)
}

#[test]
fn pmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 194], OperandSize::Dword)
}

#[test]
fn pmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 205745714, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 180, 122, 50, 110, 67, 12], OperandSize::Dword)
}

#[test]
fn pmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 192], OperandSize::Qword)
}

#[test]
fn pmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 1051087010, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 177, 162, 80, 166, 62], OperandSize::Qword)
}

