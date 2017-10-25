use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 239], OperandSize::Dword)
}

#[test]
fn cvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 2059483441, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 44, 141, 49, 57, 193, 122], OperandSize::Dword)
}

#[test]
fn cvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 221], OperandSize::Qword)
}

#[test]
fn cvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 35], OperandSize::Qword)
}

#[test]
fn cvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 239], OperandSize::Qword)
}

#[test]
fn cvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 51], OperandSize::Qword)
}

