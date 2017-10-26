use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 252], OperandSize::Dword)
}

#[test]
fn pblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1673428441, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 161, 217, 125, 190, 99], OperandSize::Dword)
}

#[test]
fn pblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 242], OperandSize::Qword)
}

#[test]
fn pblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RAX, 2006106051, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 144, 195, 191, 146, 119], OperandSize::Qword)
}

