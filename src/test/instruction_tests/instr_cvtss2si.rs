use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 206], OperandSize::Dword)
}

#[test]
fn cvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1376972412, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 28, 117, 124, 238, 18, 82], OperandSize::Dword)
}

#[test]
fn cvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 253], OperandSize::Qword)
}

#[test]
fn cvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 28, 222], OperandSize::Qword)
}

#[test]
fn cvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 210], OperandSize::Qword)
}

#[test]
fn cvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 232043223, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 156, 249, 215, 178, 212, 13], OperandSize::Qword)
}

