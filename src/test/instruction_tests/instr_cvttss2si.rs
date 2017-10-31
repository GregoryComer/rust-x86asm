use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 222], OperandSize::Dword)
}

#[test]
fn cvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 57], OperandSize::Dword)
}

#[test]
fn cvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 214], OperandSize::Qword)
}

#[test]
fn cvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 60, 207], OperandSize::Qword)
}

#[test]
fn cvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 241], OperandSize::Qword)
}

#[test]
fn cvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 40], OperandSize::Qword)
}

