use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 241], OperandSize::Dword)
}

#[test]
fn cvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 40], OperandSize::Dword)
}

#[test]
fn cvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 204], OperandSize::Qword)
}

#[test]
fn cvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 31], OperandSize::Qword)
}

#[test]
fn cvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 229], OperandSize::Qword)
}

#[test]
fn cvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 2078545212, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 28, 157, 60, 21, 228, 123], OperandSize::Qword)
}

