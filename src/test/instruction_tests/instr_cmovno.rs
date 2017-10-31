use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovno_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 252], OperandSize::Word)
}

#[test]
fn cmovno_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(SI, 109, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 108, 109], OperandSize::Word)
}

#[test]
fn cmovno_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 235], OperandSize::Dword)
}

#[test]
fn cmovno_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EAX, 1205360836, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 136, 196, 88, 216, 71], OperandSize::Dword)
}

#[test]
fn cmovno_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 207], OperandSize::Qword)
}

#[test]
fn cmovno_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 60, 222], OperandSize::Qword)
}

#[test]
fn cmovno_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 209], OperandSize::Word)
}

#[test]
fn cmovno_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 9], OperandSize::Word)
}

#[test]
fn cmovno_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 217], OperandSize::Dword)
}

#[test]
fn cmovno_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 73694906, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 36, 69, 186, 126, 100, 4], OperandSize::Dword)
}

#[test]
fn cmovno_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 246], OperandSize::Qword)
}

#[test]
fn cmovno_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 62], OperandSize::Qword)
}

#[test]
fn cmovno_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 246], OperandSize::Qword)
}

#[test]
fn cmovno_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 44, 194], OperandSize::Qword)
}

