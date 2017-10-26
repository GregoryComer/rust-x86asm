use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovno_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 233], OperandSize::Word)
}

#[test]
fn cmovno_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(DI, 29319, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 173, 135, 114], OperandSize::Word)
}

#[test]
fn cmovno_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 222], OperandSize::Dword)
}

#[test]
fn cmovno_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 590647241, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 28, 77, 201, 143, 52, 35], OperandSize::Dword)
}

#[test]
fn cmovno_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 243], OperandSize::Qword)
}

#[test]
fn cmovno_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1348711106, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 140, 95, 194, 178, 99, 80], OperandSize::Qword)
}

#[test]
fn cmovno_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 244], OperandSize::Word)
}

#[test]
fn cmovno_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(BX, 24079, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 151, 15, 94], OperandSize::Word)
}

#[test]
fn cmovno_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 239], OperandSize::Dword)
}

#[test]
fn cmovno_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(ESI, 1084318399, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 166, 191, 98, 161, 64], OperandSize::Dword)
}

#[test]
fn cmovno_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 222], OperandSize::Qword)
}

#[test]
fn cmovno_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 9], OperandSize::Qword)
}

#[test]
fn cmovno_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 213], OperandSize::Qword)
}

#[test]
fn cmovno_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 741951242, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 12, 245, 10, 71, 57, 44], OperandSize::Qword)
}

