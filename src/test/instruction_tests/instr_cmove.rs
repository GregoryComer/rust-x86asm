use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmove_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 220], OperandSize::Word)
}

#[test]
fn cmove_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(BX, 30518, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 159, 54, 119], OperandSize::Word)
}

#[test]
fn cmove_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 238], OperandSize::Dword)
}

#[test]
fn cmove_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 12, 214], OperandSize::Dword)
}

#[test]
fn cmove_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 254], OperandSize::Qword)
}

#[test]
fn cmove_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 30119667, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 36, 69, 243, 150, 203, 1], OperandSize::Qword)
}

#[test]
fn cmove_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 209], OperandSize::Word)
}

#[test]
fn cmove_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 4766, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 168, 158, 18], OperandSize::Word)
}

#[test]
fn cmove_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 251], OperandSize::Dword)
}

#[test]
fn cmove_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ECX)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 15], OperandSize::Dword)
}

#[test]
fn cmove_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 249], OperandSize::Qword)
}

#[test]
fn cmove_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RAX, 205076789, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 152, 53, 57, 57, 12], OperandSize::Qword)
}

#[test]
fn cmove_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 241], OperandSize::Qword)
}

#[test]
fn cmove_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 26], OperandSize::Qword)
}

