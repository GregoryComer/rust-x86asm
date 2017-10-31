use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovo_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 252], OperandSize::Word)
}

#[test]
fn cmovo_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 17807, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 154, 143, 69], OperandSize::Word)
}

#[test]
fn cmovo_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 230], OperandSize::Dword)
}

#[test]
fn cmovo_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1404492169, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 180, 190, 137, 217, 182, 83], OperandSize::Dword)
}

#[test]
fn cmovo_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 215], OperandSize::Qword)
}

#[test]
fn cmovo_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BX)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 25], OperandSize::Qword)
}

#[test]
fn cmovo_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 221], OperandSize::Word)
}

#[test]
fn cmovo_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDX)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 21], OperandSize::Word)
}

#[test]
fn cmovo_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 201], OperandSize::Dword)
}

#[test]
fn cmovo_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 43], OperandSize::Dword)
}

#[test]
fn cmovo_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 229], OperandSize::Qword)
}

#[test]
fn cmovo_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1609294212, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 188, 187, 132, 225, 235, 95], OperandSize::Qword)
}

#[test]
fn cmovo_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 215], OperandSize::Qword)
}

#[test]
fn cmovo_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1130757799, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 148, 240, 167, 254, 101, 67], OperandSize::Qword)
}

