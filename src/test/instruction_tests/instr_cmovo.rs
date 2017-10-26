use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovo_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 201], OperandSize::Word)
}

#[test]
fn cmovo_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 43], OperandSize::Word)
}

#[test]
fn cmovo_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 250], OperandSize::Dword)
}

#[test]
fn cmovo_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 478898744, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 60, 253, 56, 106, 139, 28], OperandSize::Dword)
}

#[test]
fn cmovo_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 233], OperandSize::Qword)
}

#[test]
fn cmovo_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(CX)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 9], OperandSize::Qword)
}

#[test]
fn cmovo_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 219], OperandSize::Word)
}

#[test]
fn cmovo_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 239, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 187, 239, 0], OperandSize::Word)
}

#[test]
fn cmovo_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 237], OperandSize::Dword)
}

#[test]
fn cmovo_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 19], OperandSize::Dword)
}

#[test]
fn cmovo_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 241], OperandSize::Qword)
}

#[test]
fn cmovo_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 34], OperandSize::Qword)
}

#[test]
fn cmovo_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 219], OperandSize::Qword)
}

#[test]
fn cmovo_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 54], OperandSize::Qword)
}

