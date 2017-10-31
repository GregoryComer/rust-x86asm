use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 205], OperandSize::Word)
}

#[test]
fn cmovc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 168, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 177, 168, 0], OperandSize::Word)
}

#[test]
fn cmovc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 228], OperandSize::Dword)
}

#[test]
fn cmovc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 28, 251], OperandSize::Dword)
}

#[test]
fn cmovc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 228], OperandSize::Qword)
}

#[test]
fn cmovc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1294142833, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 156, 129, 113, 13, 35, 77], OperandSize::Qword)
}

#[test]
fn cmovc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 241], OperandSize::Word)
}

#[test]
fn cmovc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBX)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 28], OperandSize::Word)
}

#[test]
fn cmovc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 229], OperandSize::Dword)
}

#[test]
fn cmovc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 467139076, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 52, 117, 4, 250, 215, 27], OperandSize::Dword)
}

#[test]
fn cmovc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 234], OperandSize::Qword)
}

#[test]
fn cmovc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RBX, 2143735325, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 171, 29, 206, 198, 127], OperandSize::Qword)
}

#[test]
fn cmovc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 212], OperandSize::Qword)
}

#[test]
fn cmovc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1539140717, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 44, 141, 109, 108, 189, 91], OperandSize::Qword)
}

