use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 207], OperandSize::Dword)
}

#[test]
fn cvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 58], OperandSize::Dword)
}

#[test]
fn cvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 202], OperandSize::Qword)
}

#[test]
fn cvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1683053462, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 60, 117, 150, 91, 81, 100], OperandSize::Qword)
}

#[test]
fn cvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 220], OperandSize::Qword)
}

#[test]
fn cvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1443948384, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 60, 197, 96, 231, 16, 86], OperandSize::Qword)
}

