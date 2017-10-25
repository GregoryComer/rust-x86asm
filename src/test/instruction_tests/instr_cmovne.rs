use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 209], OperandSize::Word)
}

#[test]
fn cmovne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(SI)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 55], OperandSize::Word)
}

#[test]
fn cmovne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 219], OperandSize::Dword)
}

#[test]
fn cmovne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 516523469, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 156, 67, 205, 133, 201, 30], OperandSize::Dword)
}

#[test]
fn cmovne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 229], OperandSize::Qword)
}

#[test]
fn cmovne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(SP)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 33], OperandSize::Qword)
}

#[test]
fn cmovne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 239], OperandSize::Word)
}

#[test]
fn cmovne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 126, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 99, 126], OperandSize::Word)
}

#[test]
fn cmovne_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 223], OperandSize::Dword)
}

#[test]
fn cmovne_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 881083881, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 52, 77, 233, 69, 132, 52], OperandSize::Dword)
}

#[test]
fn cmovne_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 245], OperandSize::Qword)
}

#[test]
fn cmovne_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 36, 201], OperandSize::Qword)
}

#[test]
fn cmovne_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 223], OperandSize::Qword)
}

#[test]
fn cmovne_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1831749649, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 12, 245, 17, 72, 46, 109], OperandSize::Qword)
}

