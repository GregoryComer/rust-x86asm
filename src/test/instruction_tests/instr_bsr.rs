use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 203], OperandSize::Word)
}

#[test]
fn bsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BP, 7937, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 190, 1, 31], OperandSize::Word)
}

#[test]
fn bsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 252], OperandSize::Dword)
}

#[test]
fn bsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1556279372, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 12, 181, 76, 240, 194, 92], OperandSize::Dword)
}

#[test]
fn bsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 239], OperandSize::Qword)
}

#[test]
fn bsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 12, 251], OperandSize::Qword)
}

#[test]
fn bsr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 226], OperandSize::Word)
}

#[test]
fn bsr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 13857, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 170, 33, 54], OperandSize::Word)
}

#[test]
fn bsr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 254], OperandSize::Dword)
}

#[test]
fn bsr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EBX, 2037493441, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 187, 193, 174, 113, 121], OperandSize::Dword)
}

#[test]
fn bsr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 255], OperandSize::Qword)
}

#[test]
fn bsr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 60, 144], OperandSize::Qword)
}

#[test]
fn bsr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 231], OperandSize::Qword)
}

#[test]
fn bsr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RDX, 554109961, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 146, 9, 12, 7, 33], OperandSize::Qword)
}

