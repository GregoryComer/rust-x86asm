use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 222], OperandSize::Word)
}

#[test]
fn cvttps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(SI, 5473, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 140, 97, 21], OperandSize::Word)
}

#[test]
fn cvttps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 194], OperandSize::Dword)
}

#[test]
fn cvttps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 24], OperandSize::Dword)
}

#[test]
fn cvttps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 229], OperandSize::Qword)
}

#[test]
fn cvttps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1739330134, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 188, 198, 86, 18, 172, 103], OperandSize::Qword)
}

