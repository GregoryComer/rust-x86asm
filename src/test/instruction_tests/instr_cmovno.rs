use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovno_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 225], OperandSize::Word)
}

#[test]
fn cmovno_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(BP, 55, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 78, 55], OperandSize::Word)
}

#[test]
fn cmovno_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 201], OperandSize::Dword)
}

#[test]
fn cmovno_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 531988756, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 140, 251, 20, 129, 181, 31], OperandSize::Dword)
}

#[test]
fn cmovno_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 203], OperandSize::Qword)
}

#[test]
fn cmovno_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RDX, 2123396002, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 186, 162, 115, 144, 126], OperandSize::Qword)
}

#[test]
fn cmovno_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 228], OperandSize::Word)
}

#[test]
fn cmovno_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BX, 170, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 143, 170, 0], OperandSize::Word)
}

#[test]
fn cmovno_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 217], OperandSize::Dword)
}

#[test]
fn cmovno_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EAX, 979680237, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 152, 237, 187, 100, 58], OperandSize::Dword)
}

#[test]
fn cmovno_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 203], OperandSize::Qword)
}

#[test]
fn cmovno_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RBX, 263843448, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 147, 120, 238, 185, 15], OperandSize::Qword)
}

#[test]
fn cmovno_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 239], OperandSize::Qword)
}

#[test]
fn cmovno_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 31193992, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 156, 75, 136, 251, 219, 1], OperandSize::Qword)
}

