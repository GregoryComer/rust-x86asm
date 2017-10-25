use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 241], OperandSize::Word)
}

#[test]
fn cvttps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 8454, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 163, 6, 33], OperandSize::Word)
}

#[test]
fn cvttps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 203], OperandSize::Dword)
}

#[test]
fn cvttps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EAX, 678228297, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 184, 73, 241, 108, 40], OperandSize::Dword)
}

#[test]
fn cvttps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 230], OperandSize::Qword)
}

#[test]
fn cvttps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RDX, 1534141444, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 138, 4, 36, 113, 91], OperandSize::Qword)
}

