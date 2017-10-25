use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM4)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 229, 73], OperandSize::Dword)
}

#[test]
fn pinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 0, 92], OperandSize::Dword)
}

#[test]
fn pinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM2)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 215, 13], OperandSize::Qword)
}

#[test]
fn pinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 302183207, Some(OperandSize::Word), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 12, 221, 39, 243, 2, 18, 118], OperandSize::Qword)
}

#[test]
fn pinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 247, 47], OperandSize::Dword)
}

#[test]
fn pinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1786118837, Some(OperandSize::Word), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 172, 147, 181, 2, 118, 106, 120], OperandSize::Dword)
}

#[test]
fn pinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 229, 31], OperandSize::Qword)
}

#[test]
fn pinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RBX, 13955724, Some(OperandSize::Word), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 131, 140, 242, 212, 0, 72], OperandSize::Qword)
}

