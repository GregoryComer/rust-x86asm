use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND1)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 206], OperandSize::Dword)
}

#[test]
fn bndcn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1569418478, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 132, 203, 238, 108, 139, 93], OperandSize::Dword)
}

#[test]
fn bndcn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND0)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 195], OperandSize::Qword)
}

#[test]
fn bndcn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 20, 114], OperandSize::Qword)
}

