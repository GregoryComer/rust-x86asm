use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 245, 48], OperandSize::Dword)
}

#[test]
fn pinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 1467451777, Some(OperandSize::Dword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 151, 129, 137, 119, 87, 94], OperandSize::Dword)
}

#[test]
fn pinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 249, 116], OperandSize::Qword)
}

#[test]
fn pinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1559537244, Some(OperandSize::Dword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 44, 149, 92, 166, 244, 92, 118], OperandSize::Qword)
}

