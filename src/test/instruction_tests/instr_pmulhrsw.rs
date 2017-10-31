use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 230], OperandSize::Dword)
}

#[test]
fn pmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 4, 128], OperandSize::Dword)
}

#[test]
fn pmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 211], OperandSize::Qword)
}

#[test]
fn pmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RSI, 2034886433, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 150, 33, 231, 73, 121], OperandSize::Qword)
}

#[test]
fn pmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 198], OperandSize::Dword)
}

#[test]
fn pmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 364882187, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 172, 78, 11, 169, 191, 21], OperandSize::Dword)
}

#[test]
fn pmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 211], OperandSize::Qword)
}

#[test]
fn pmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 59], OperandSize::Qword)
}

