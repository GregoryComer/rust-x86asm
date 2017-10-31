use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmove_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 206], OperandSize::Word)
}

#[test]
fn cmove_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(DI, 11670, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 141, 150, 45], OperandSize::Word)
}

#[test]
fn cmove_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 222], OperandSize::Dword)
}

#[test]
fn cmove_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 35], OperandSize::Dword)
}

#[test]
fn cmove_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 206], OperandSize::Qword)
}

#[test]
fn cmove_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RCX, 1966765746, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 185, 178, 118, 58, 117], OperandSize::Qword)
}

#[test]
fn cmove_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 215], OperandSize::Word)
}

#[test]
fn cmove_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 39], OperandSize::Word)
}

#[test]
fn cmove_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 241], OperandSize::Dword)
}

#[test]
fn cmove_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 393237593, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 52, 133, 89, 84, 112, 23], OperandSize::Dword)
}

#[test]
fn cmove_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 214], OperandSize::Qword)
}

#[test]
fn cmove_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 44, 178], OperandSize::Qword)
}

#[test]
fn cmove_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 210], OperandSize::Qword)
}

#[test]
fn cmove_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 23], OperandSize::Qword)
}

