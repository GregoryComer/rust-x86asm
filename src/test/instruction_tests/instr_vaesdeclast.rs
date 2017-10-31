use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 223, 242], OperandSize::Dword)
}

#[test]
fn vaesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1069944002, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 223, 12, 133, 194, 12, 198, 63], OperandSize::Dword)
}

#[test]
fn vaesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 223, 219], OperandSize::Qword)
}

#[test]
fn vaesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDECLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 223, 2], OperandSize::Qword)
}

