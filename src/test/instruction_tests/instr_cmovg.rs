use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 213], OperandSize::Word)
}

#[test]
fn cmovg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 24], OperandSize::Word)
}

#[test]
fn cmovg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 245], OperandSize::Dword)
}

#[test]
fn cmovg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 994017575, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 148, 88, 39, 129, 63, 59], OperandSize::Dword)
}

#[test]
fn cmovg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 250], OperandSize::Qword)
}

#[test]
fn cmovg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RAX, 1606429334, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 136, 150, 42, 192, 95], OperandSize::Qword)
}

#[test]
fn cmovg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 206], OperandSize::Word)
}

#[test]
fn cmovg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BX, 235, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 143, 235, 0], OperandSize::Word)
}

#[test]
fn cmovg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 233], OperandSize::Dword)
}

#[test]
fn cmovg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 994440856, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 188, 89, 152, 246, 69, 59], OperandSize::Dword)
}

#[test]
fn cmovg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 230], OperandSize::Qword)
}

#[test]
fn cmovg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1485522551, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 140, 70, 119, 70, 139, 88], OperandSize::Qword)
}

#[test]
fn cmovg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 226], OperandSize::Qword)
}

#[test]
fn cmovg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 28, 86], OperandSize::Qword)
}

