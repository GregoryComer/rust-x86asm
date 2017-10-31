use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 239], OperandSize::Word)
}

#[test]
fn cvttps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(BX, 1257, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 175, 233, 4], OperandSize::Word)
}

#[test]
fn cvttps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 227], OperandSize::Dword)
}

#[test]
fn cvttps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 877936780, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 52, 69, 140, 64, 84, 52], OperandSize::Dword)
}

#[test]
fn cvttps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 229], OperandSize::Qword)
}

#[test]
fn cvttps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 39], OperandSize::Qword)
}

