use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 202], OperandSize::Dword)
}

#[test]
fn cvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ECX, 1801789437, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 153, 253, 31, 101, 107], OperandSize::Dword)
}

#[test]
fn cvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 244], OperandSize::Qword)
}

#[test]
fn cvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1736746752, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 28, 85, 0, 167, 132, 103], OperandSize::Qword)
}

#[test]
fn cvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 206], OperandSize::Qword)
}

#[test]
fn cvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 52, 190], OperandSize::Qword)
}

