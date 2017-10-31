use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lsl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 206], OperandSize::Word)
}

#[test]
fn lsl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 8], OperandSize::Word)
}

#[test]
fn lsl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 239], OperandSize::Dword)
}

#[test]
fn lsl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(ECX, 1012175554, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 169, 194, 146, 84, 60], OperandSize::Dword)
}

#[test]
fn lsl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 245], OperandSize::Qword)
}

#[test]
fn lsl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1960856650, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 36, 213, 74, 76, 224, 116], OperandSize::Qword)
}

#[test]
fn lsl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 213], OperandSize::Word)
}

#[test]
fn lsl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(DI, 76, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 125, 76], OperandSize::Word)
}

#[test]
fn lsl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 206], OperandSize::Dword)
}

#[test]
fn lsl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EDI, 118509328, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 191, 16, 79, 16, 7], OperandSize::Dword)
}

#[test]
fn lsl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 227], OperandSize::Qword)
}

#[test]
fn lsl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(RSI, 1997799156, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 142, 244, 254, 19, 119], OperandSize::Qword)
}

#[test]
fn lsl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 223], OperandSize::Qword)
}

#[test]
fn lsl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1628479483, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 36, 77, 251, 159, 16, 97], OperandSize::Qword)
}

