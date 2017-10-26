use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn comiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 254], OperandSize::Dword)
}

#[test]
fn comiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 52, 66], OperandSize::Dword)
}

#[test]
fn comiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 215], OperandSize::Qword)
}

#[test]
fn comiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 15], OperandSize::Qword)
}

