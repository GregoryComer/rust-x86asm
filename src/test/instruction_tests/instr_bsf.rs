use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsf_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 237], OperandSize::Word)
}

#[test]
fn bsf_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(SI, 13231, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 140, 175, 51], OperandSize::Word)
}

#[test]
fn bsf_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 228], OperandSize::Dword)
}

#[test]
fn bsf_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 236452888, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 164, 214, 24, 252, 23, 14], OperandSize::Dword)
}

#[test]
fn bsf_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 220], OperandSize::Qword)
}

#[test]
fn bsf_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SI)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 51], OperandSize::Qword)
}

#[test]
fn bsf_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 238], OperandSize::Word)
}

#[test]
fn bsf_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBP)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 47], OperandSize::Word)
}

#[test]
fn bsf_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 212], OperandSize::Dword)
}

#[test]
fn bsf_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 12, 240], OperandSize::Dword)
}

#[test]
fn bsf_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 247], OperandSize::Qword)
}

#[test]
fn bsf_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 825348798, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 28, 189, 190, 210, 49, 49], OperandSize::Qword)
}

#[test]
fn bsf_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 228], OperandSize::Qword)
}

#[test]
fn bsf_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1514573573, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 156, 90, 5, 143, 70, 90], OperandSize::Qword)
}

