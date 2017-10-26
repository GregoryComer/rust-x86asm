use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 217], OperandSize::Dword)
}

#[test]
fn cvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(ECX, 2117346834, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 137, 18, 38, 52, 126], OperandSize::Dword)
}

#[test]
fn cvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 200], OperandSize::Qword)
}

#[test]
fn cvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 195619564, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 180, 142, 236, 234, 168, 11], OperandSize::Qword)
}

#[test]
fn cvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 234], OperandSize::Qword)
}

#[test]
fn cvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(IndirectDisplaced(RBX, 579580940, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 163, 12, 180, 139, 34], OperandSize::Qword)
}

