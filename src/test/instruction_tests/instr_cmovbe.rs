use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 238], OperandSize::Word)
}

#[test]
fn cmovbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(BP, 136, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 174, 136, 0], OperandSize::Word)
}

#[test]
fn cmovbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 230], OperandSize::Dword)
}

#[test]
fn cmovbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SI)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 50], OperandSize::Dword)
}

#[test]
fn cmovbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 249], OperandSize::Qword)
}

#[test]
fn cmovbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1002504757, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 52, 85, 53, 2, 193, 59], OperandSize::Qword)
}

#[test]
fn cmovbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 214], OperandSize::Word)
}

#[test]
fn cmovbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(BP, 8290, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 150, 98, 32], OperandSize::Word)
}

#[test]
fn cmovbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 249], OperandSize::Dword)
}

#[test]
fn cmovbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 19], OperandSize::Dword)
}

#[test]
fn cmovbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 225], OperandSize::Qword)
}

#[test]
fn cmovbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1124330706, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 164, 210, 210, 236, 3, 67], OperandSize::Qword)
}

#[test]
fn cmovbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 202], OperandSize::Qword)
}

#[test]
fn cmovbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1711369084, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 188, 66, 124, 107, 1, 102], OperandSize::Qword)
}

