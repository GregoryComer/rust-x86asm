use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 236], OperandSize::Dword)
}

#[test]
fn cvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1710820088, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 52, 69, 248, 10, 249, 101], OperandSize::Dword)
}

#[test]
fn cvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 215], OperandSize::Qword)
}

#[test]
fn cvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RDI, 801227931, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 151, 155, 196, 193, 47], OperandSize::Qword)
}

#[test]
fn cvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 238], OperandSize::Qword)
}

#[test]
fn cvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 52, 65], OperandSize::Qword)
}

