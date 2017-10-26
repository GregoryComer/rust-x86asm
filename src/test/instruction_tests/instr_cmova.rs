use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmova_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 251], OperandSize::Word)
}

#[test]
fn cmova_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(DI, 31, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 117, 31], OperandSize::Word)
}

#[test]
fn cmova_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 231], OperandSize::Dword)
}

#[test]
fn cmova_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DX)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 18], OperandSize::Dword)
}

#[test]
fn cmova_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 250], OperandSize::Qword)
}

#[test]
fn cmova_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(RDX, 2039475576, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 154, 120, 237, 143, 121], OperandSize::Qword)
}

#[test]
fn cmova_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 242], OperandSize::Word)
}

#[test]
fn cmova_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 41], OperandSize::Word)
}

#[test]
fn cmova_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 241], OperandSize::Dword)
}

#[test]
fn cmova_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1912434511, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 36, 205, 79, 111, 253, 113], OperandSize::Dword)
}

#[test]
fn cmova_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 231], OperandSize::Qword)
}

#[test]
fn cmova_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 404784622, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 36, 77, 238, 133, 32, 24], OperandSize::Qword)
}

#[test]
fn cmova_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 225], OperandSize::Qword)
}

#[test]
fn cmova_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 39], OperandSize::Qword)
}

