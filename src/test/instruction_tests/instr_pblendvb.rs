use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 197], OperandSize::Dword)
}

#[test]
fn pblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 7], OperandSize::Dword)
}

#[test]
fn pblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 196], OperandSize::Qword)
}

#[test]
fn pblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1220829008, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 60, 69, 80, 95, 196, 72], OperandSize::Qword)
}

