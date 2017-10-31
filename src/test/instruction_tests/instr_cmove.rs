use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmove_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 201], OperandSize::Word)
}

#[test]
fn cmove_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(CX)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 15], OperandSize::Word)
}

#[test]
fn cmove_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 227], OperandSize::Dword)
}

#[test]
fn cmove_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 36, 223], OperandSize::Dword)
}

#[test]
fn cmove_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 231], OperandSize::Qword)
}

#[test]
fn cmove_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(RDX, 762037794, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 170, 34, 198, 107, 45], OperandSize::Qword)
}

#[test]
fn cmove_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 223], OperandSize::Word)
}

#[test]
fn cmove_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 43], OperandSize::Word)
}

#[test]
fn cmove_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 204], OperandSize::Dword)
}

#[test]
fn cmove_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2125682134, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 36, 197, 214, 85, 179, 126], OperandSize::Dword)
}

#[test]
fn cmove_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 205], OperandSize::Qword)
}

#[test]
fn cmove_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1901985406, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 44, 125, 126, 254, 93, 113], OperandSize::Qword)
}

#[test]
fn cmove_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 244], OperandSize::Qword)
}

#[test]
fn cmove_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 28, 241], OperandSize::Qword)
}

