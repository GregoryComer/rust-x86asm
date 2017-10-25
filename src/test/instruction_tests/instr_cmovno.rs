use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovno_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 238], OperandSize::Word)
}

#[test]
fn cmovno_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 202, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 168, 202, 0], OperandSize::Word)
}

#[test]
fn cmovno_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 222], OperandSize::Dword)
}

#[test]
fn cmovno_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 502947289, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 20, 221, 217, 93, 250, 29], OperandSize::Dword)
}

#[test]
fn cmovno_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 254], OperandSize::Qword)
}

#[test]
fn cmovno_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RCX, 2097092523, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 137, 171, 23, 255, 124], OperandSize::Qword)
}

#[test]
fn cmovno_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 227], OperandSize::Word)
}

#[test]
fn cmovno_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(DI, 183, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 157, 183, 0], OperandSize::Word)
}

#[test]
fn cmovno_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 218], OperandSize::Dword)
}

#[test]
fn cmovno_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 18], OperandSize::Dword)
}

#[test]
fn cmovno_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 255], OperandSize::Qword)
}

#[test]
fn cmovno_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 2065075892, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 156, 207, 180, 142, 22, 123], OperandSize::Qword)
}

#[test]
fn cmovno_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 215], OperandSize::Qword)
}

#[test]
fn cmovno_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RDX, 1211661276, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 178, 220, 123, 56, 72], OperandSize::Qword)
}

