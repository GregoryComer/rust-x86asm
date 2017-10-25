use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND3)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 221], OperandSize::Dword)
}

#[test]
fn bndcn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND2)), operand2: Some(IndirectDisplaced(EAX, 1263110663, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 144, 7, 138, 73, 75], OperandSize::Dword)
}

#[test]
fn bndcn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND0)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 196], OperandSize::Qword)
}

#[test]
fn bndcn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND0)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 1], OperandSize::Qword)
}

