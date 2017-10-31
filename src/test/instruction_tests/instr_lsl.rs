use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lsl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 203], OperandSize::Word)
}

#[test]
fn lsl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 31363, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 160, 131, 122], OperandSize::Word)
}

#[test]
fn lsl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 233], OperandSize::Dword)
}

#[test]
fn lsl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EBX, 1467972489, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 187, 137, 123, 127, 87], OperandSize::Dword)
}

#[test]
fn lsl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 234], OperandSize::Qword)
}

#[test]
fn lsl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 2025446526, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 12, 213, 126, 220, 185, 120], OperandSize::Qword)
}

#[test]
fn lsl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 244], OperandSize::Word)
}

#[test]
fn lsl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 49], OperandSize::Word)
}

#[test]
fn lsl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 220], OperandSize::Dword)
}

#[test]
fn lsl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(ECX, 235894944, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 137, 160, 120, 15, 14], OperandSize::Dword)
}

#[test]
fn lsl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 246], OperandSize::Qword)
}

#[test]
fn lsl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 183480465, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 164, 190, 145, 176, 239, 10], OperandSize::Qword)
}

#[test]
fn lsl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 218], OperandSize::Qword)
}

#[test]
fn lsl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RDX, 2107830226, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 170, 210, 239, 162, 125], OperandSize::Qword)
}

