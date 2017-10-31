use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 205], OperandSize::Word)
}

#[test]
fn cmovna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 18], OperandSize::Word)
}

#[test]
fn cmovna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 209], OperandSize::Dword)
}

#[test]
fn cmovna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(BX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 24], OperandSize::Dword)
}

#[test]
fn cmovna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 255], OperandSize::Qword)
}

#[test]
fn cmovna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 12, 250], OperandSize::Qword)
}

#[test]
fn cmovna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 228], OperandSize::Word)
}

#[test]
fn cmovna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(Memory(27793, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 54, 145, 108], OperandSize::Word)
}

#[test]
fn cmovna_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 231], OperandSize::Dword)
}

#[test]
fn cmovna_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 52, 94], OperandSize::Dword)
}

#[test]
fn cmovna_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 220], OperandSize::Qword)
}

#[test]
fn cmovna_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 50], OperandSize::Qword)
}

#[test]
fn cmovna_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 254], OperandSize::Qword)
}

#[test]
fn cmovna_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RBX, 2115663028, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 155, 180, 116, 26, 126], OperandSize::Qword)
}

