use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 211], OperandSize::Word)
}

#[test]
fn bsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SI)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 53], OperandSize::Word)
}

#[test]
fn bsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 252], OperandSize::Dword)
}

#[test]
fn bsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EBX, 399005178, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 187, 250, 85, 200, 23], OperandSize::Dword)
}

#[test]
fn bsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 230], OperandSize::Qword)
}

#[test]
fn bsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 12, 191], OperandSize::Qword)
}

#[test]
fn bsr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 205], OperandSize::Word)
}

#[test]
fn bsr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 15], OperandSize::Word)
}

#[test]
fn bsr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 228], OperandSize::Dword)
}

#[test]
fn bsr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1885637671, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 36, 85, 39, 140, 100, 112], OperandSize::Dword)
}

#[test]
fn bsr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 247], OperandSize::Qword)
}

#[test]
fn bsr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 982129245, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 172, 255, 93, 26, 138, 58], OperandSize::Qword)
}

#[test]
fn bsr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 235], OperandSize::Qword)
}

#[test]
fn bsr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RDI, 1160332086, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 191, 54, 67, 41, 69], OperandSize::Qword)
}

