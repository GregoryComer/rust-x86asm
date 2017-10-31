use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND2)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 213], OperandSize::Dword)
}

#[test]
fn bndcn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1695177414, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 20, 205, 198, 90, 10, 101], OperandSize::Dword)
}

#[test]
fn bndcn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND2)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 210], OperandSize::Qword)
}

#[test]
fn bndcn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1412788606, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 28, 189, 126, 113, 53, 84], OperandSize::Qword)
}

