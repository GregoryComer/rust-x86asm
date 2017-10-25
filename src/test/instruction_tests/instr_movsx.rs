use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(DX)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 211], OperandSize::Word)
}

#[test]
fn movsx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SP)), operand2: Some(Memory(5503, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 38, 127, 21], OperandSize::Word)
}

#[test]
fn movsx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 243], OperandSize::Dword)
}

#[test]
fn movsx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 567472470, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 164, 192, 86, 241, 210, 33], OperandSize::Dword)
}

#[test]
fn movsx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 209], OperandSize::Qword)
}

#[test]
fn movsx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 60, 73], OperandSize::Qword)
}

#[test]
fn movsx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ECX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 202], OperandSize::Word)
}

#[test]
fn movsx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(SI, 124, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 124, 124], OperandSize::Word)
}

#[test]
fn movsx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 227], OperandSize::Dword)
}

#[test]
fn movsx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 42], OperandSize::Dword)
}

#[test]
fn movsx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 250], OperandSize::Qword)
}

#[test]
fn movsx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RDI, 1751828563, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 183, 83, 200, 106, 104], OperandSize::Qword)
}

#[test]
fn movsx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RSI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 243], OperandSize::Qword)
}

#[test]
fn movsx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 26], OperandSize::Qword)
}

#[test]
fn movsx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 251], OperandSize::Word)
}

#[test]
fn movsx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 43], OperandSize::Word)
}

#[test]
fn movsx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 243], OperandSize::Dword)
}

#[test]
fn movsx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1238631698, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 164, 178, 18, 5, 212, 73], OperandSize::Dword)
}

#[test]
fn movsx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 215], OperandSize::Qword)
}

#[test]
fn movsx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1688752842, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 12, 69, 202, 82, 168, 100], OperandSize::Qword)
}

#[test]
fn movsx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RDI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 253], OperandSize::Qword)
}

#[test]
fn movsx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 36, 121], OperandSize::Qword)
}

