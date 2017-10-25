use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 242], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 62], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 254], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 43], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 196], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 28, 120], OperandSize::Qword)
}

