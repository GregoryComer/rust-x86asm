use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 200], OperandSize::Dword)
}

#[test]
fn pcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EDI, 174415315, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 159, 211, 93, 101, 10], OperandSize::Dword)
}

#[test]
fn pcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 245], OperandSize::Qword)
}

#[test]
fn pcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 4, 246], OperandSize::Qword)
}

#[test]
fn pcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 197], OperandSize::Dword)
}

#[test]
fn pcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 34], OperandSize::Dword)
}

#[test]
fn pcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 225], OperandSize::Qword)
}

#[test]
fn pcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 52, 153], OperandSize::Qword)
}

