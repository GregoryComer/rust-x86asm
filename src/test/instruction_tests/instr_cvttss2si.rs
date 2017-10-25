use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 223], OperandSize::Dword)
}

#[test]
fn cvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 30], OperandSize::Dword)
}

#[test]
fn cvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 243], OperandSize::Qword)
}

#[test]
fn cvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 142609355, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 20, 125, 203, 11, 128, 8], OperandSize::Qword)
}

#[test]
fn cvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 216], OperandSize::Qword)
}

#[test]
fn cvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 849766248, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 156, 190, 104, 103, 166, 50], OperandSize::Qword)
}

