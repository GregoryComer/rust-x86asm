use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmova_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 213], OperandSize::Word)
}

#[test]
fn cmova_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 123, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 75, 123], OperandSize::Word)
}

#[test]
fn cmova_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 223], OperandSize::Dword)
}

#[test]
fn cmova_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1840094457, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 140, 249, 249, 156, 173, 109], OperandSize::Dword)
}

#[test]
fn cmova_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 219], OperandSize::Qword)
}

#[test]
fn cmova_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1955920503, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 164, 128, 119, 250, 148, 116], OperandSize::Qword)
}

#[test]
fn cmova_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 246], OperandSize::Word)
}

#[test]
fn cmova_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(SI, 27233, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 140, 97, 106], OperandSize::Word)
}

#[test]
fn cmova_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 202], OperandSize::Dword)
}

#[test]
fn cmova_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(ECX, 399901269, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 185, 85, 2, 214, 23], OperandSize::Dword)
}

#[test]
fn cmova_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 225], OperandSize::Qword)
}

#[test]
fn cmova_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RAX, 779961022, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 144, 190, 66, 125, 46], OperandSize::Qword)
}

#[test]
fn cmova_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 229], OperandSize::Qword)
}

#[test]
fn cmova_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RCX, 1199533098, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 137, 42, 108, 127, 71], OperandSize::Qword)
}

