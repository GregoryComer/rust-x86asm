use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 215], OperandSize::Word)
}

#[test]
fn cmovne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(BP, 18, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 86, 18], OperandSize::Word)
}

#[test]
fn cmovne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 254], OperandSize::Dword)
}

#[test]
fn cmovne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 2103428818, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 156, 241, 210, 198, 95, 125], OperandSize::Dword)
}

#[test]
fn cmovne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 212], OperandSize::Qword)
}

#[test]
fn cmovne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 60, 123], OperandSize::Qword)
}

#[test]
fn cmovne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 221], OperandSize::Word)
}

#[test]
fn cmovne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 50], OperandSize::Word)
}

#[test]
fn cmovne_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 210], OperandSize::Dword)
}

#[test]
fn cmovne_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 52, 208], OperandSize::Dword)
}

#[test]
fn cmovne_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 222], OperandSize::Qword)
}

#[test]
fn cmovne_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1733261107, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 172, 254, 51, 119, 79, 103], OperandSize::Qword)
}

#[test]
fn cmovne_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 223], OperandSize::Qword)
}

#[test]
fn cmovne_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1141112921, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 188, 246, 89, 0, 4, 68], OperandSize::Qword)
}

