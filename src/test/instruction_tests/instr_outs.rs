use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn outs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Word)
}

#[test]
fn outs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Dword)
}

#[test]
fn outs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Qword)
}

#[test]
fn outs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(SI, 138, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Word)
}

#[test]
fn outs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1226370799, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Dword)
}

#[test]
fn outs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 68635479, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Qword)
}

#[test]
fn outs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 22795, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Word)
}

#[test]
fn outs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 262958407, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Dword)
}

#[test]
fn outs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1433180202, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Qword)
}

