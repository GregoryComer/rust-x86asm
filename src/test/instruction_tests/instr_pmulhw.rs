use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 197], OperandSize::Dword)
}

#[test]
fn pmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 62], OperandSize::Dword)
}

#[test]
fn pmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 235], OperandSize::Qword)
}

#[test]
fn pmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RBX, 1640719421, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 171, 61, 100, 203, 97], OperandSize::Qword)
}

#[test]
fn pmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 201], OperandSize::Dword)
}

#[test]
fn pmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1597215631, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 156, 87, 143, 147, 51, 95], OperandSize::Dword)
}

#[test]
fn pmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 231], OperandSize::Qword)
}

#[test]
fn pmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 209339303, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 4, 117, 167, 67, 122, 12], OperandSize::Qword)
}

