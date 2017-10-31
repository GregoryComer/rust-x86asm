use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 219], OperandSize::Dword)
}

#[test]
fn cvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(EAX, 867790605, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 136, 13, 111, 185, 51], OperandSize::Dword)
}

#[test]
fn cvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 217], OperandSize::Qword)
}

#[test]
fn cvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 58], OperandSize::Qword)
}

#[test]
fn cvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 224], OperandSize::Qword)
}

#[test]
fn cvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 20, 94], OperandSize::Qword)
}

