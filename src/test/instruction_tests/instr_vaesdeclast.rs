use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 223, 223], OperandSize::Dword)
}

#[test]
fn vaesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 223, 36, 83], OperandSize::Dword)
}

#[test]
fn vaesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 223, 207], OperandSize::Qword)
}

#[test]
fn vaesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 1743258292, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 223, 159, 180, 2, 232, 103], OperandSize::Qword)
}

