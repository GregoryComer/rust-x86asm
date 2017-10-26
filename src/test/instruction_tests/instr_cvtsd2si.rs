use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 209], OperandSize::Dword)
}

#[test]
fn cvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 17], OperandSize::Dword)
}

#[test]
fn cvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 207], OperandSize::Qword)
}

#[test]
fn cvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 46], OperandSize::Qword)
}

#[test]
fn cvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 221], OperandSize::Qword)
}

#[test]
fn cvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 20, 150], OperandSize::Qword)
}

