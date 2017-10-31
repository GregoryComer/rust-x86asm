use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 255], OperandSize::Word)
}

#[test]
fn bsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SI)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 52], OperandSize::Word)
}

#[test]
fn bsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 209], OperandSize::Dword)
}

#[test]
fn bsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 948579498, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 20, 85, 170, 44, 138, 56], OperandSize::Dword)
}

#[test]
fn bsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 242], OperandSize::Qword)
}

#[test]
fn bsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 729922177, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 44, 149, 129, 186, 129, 43], OperandSize::Qword)
}

#[test]
fn bsr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 204], OperandSize::Word)
}

#[test]
fn bsr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 151, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 139, 151, 0], OperandSize::Word)
}

#[test]
fn bsr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 204], OperandSize::Dword)
}

#[test]
fn bsr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EAX, 1509329693, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 168, 29, 139, 246, 89], OperandSize::Dword)
}

#[test]
fn bsr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 207], OperandSize::Qword)
}

#[test]
fn bsr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1038273650, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 12, 85, 114, 204, 226, 61], OperandSize::Qword)
}

#[test]
fn bsr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 238], OperandSize::Qword)
}

#[test]
fn bsr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 499283091, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 156, 88, 147, 116, 194, 29], OperandSize::Qword)
}

