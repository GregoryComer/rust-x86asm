use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lar_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 219], OperandSize::Word)
}

#[test]
fn lar_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 7056, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 169, 144, 27], OperandSize::Word)
}

#[test]
fn lar_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 251], OperandSize::Dword)
}

#[test]
fn lar_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(SP)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 35], OperandSize::Dword)
}

#[test]
fn lar_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 215], OperandSize::Qword)
}

#[test]
fn lar_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 347831058, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 36, 85, 18, 123, 187, 20], OperandSize::Qword)
}

#[test]
fn lar_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(DX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 213], OperandSize::Word)
}

#[test]
fn lar_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 62, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 105, 62], OperandSize::Word)
}

#[test]
fn lar_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 251], OperandSize::Dword)
}

#[test]
fn lar_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EBX, 2146952175, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 179, 239, 227, 247, 127], OperandSize::Dword)
}

#[test]
fn lar_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(RBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 2, 237], OperandSize::Qword)
}

#[test]
fn lar_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 2, 52, 200], OperandSize::Qword)
}

