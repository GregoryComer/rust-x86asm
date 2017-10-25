use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 205], OperandSize::Dword)
}

#[test]
fn pblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1121684878, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 20, 133, 142, 141, 219, 66], OperandSize::Dword)
}

#[test]
fn pblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 241], OperandSize::Qword)
}

#[test]
fn pblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 22], OperandSize::Qword)
}

