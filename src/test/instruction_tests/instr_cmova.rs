use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmova_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 219], OperandSize::Word)
}

#[test]
fn cmova_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(SI, 27901, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 164, 253, 108], OperandSize::Word)
}

#[test]
fn cmova_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 243], OperandSize::Dword)
}

#[test]
fn cmova_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 514093825, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 52, 197, 1, 115, 164, 30], OperandSize::Dword)
}

#[test]
fn cmova_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 255], OperandSize::Qword)
}

#[test]
fn cmova_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RDX, 634010515, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 186, 147, 59, 202, 37], OperandSize::Qword)
}

#[test]
fn cmova_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 244], OperandSize::Word)
}

#[test]
fn cmova_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 26], OperandSize::Word)
}

#[test]
fn cmova_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 209], OperandSize::Dword)
}

#[test]
fn cmova_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 681571816, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 148, 248, 232, 245, 159, 40], OperandSize::Dword)
}

#[test]
fn cmova_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 231], OperandSize::Qword)
}

#[test]
fn cmova_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 12, 126], OperandSize::Qword)
}

#[test]
fn cmova_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 215], OperandSize::Qword)
}

#[test]
fn cmova_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1233378063, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 148, 126, 15, 219, 131, 73], OperandSize::Qword)
}

