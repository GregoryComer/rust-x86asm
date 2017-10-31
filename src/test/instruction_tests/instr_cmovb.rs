use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 234], OperandSize::Word)
}

#[test]
fn cmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SP)), operand2: Some(Memory(6425, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 38, 25, 25], OperandSize::Word)
}

#[test]
fn cmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 239], OperandSize::Dword)
}

#[test]
fn cmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 535495392, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 180, 151, 224, 2, 235, 31], OperandSize::Dword)
}

#[test]
fn cmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 206], OperandSize::Qword)
}

#[test]
fn cmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 595224894, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 188, 179, 62, 105, 122, 35], OperandSize::Qword)
}

#[test]
fn cmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 255], OperandSize::Word)
}

#[test]
fn cmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 15, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 73, 15], OperandSize::Word)
}

#[test]
fn cmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 249], OperandSize::Dword)
}

#[test]
fn cmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1589970965, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 180, 215, 21, 8, 197, 94], OperandSize::Dword)
}

#[test]
fn cmovb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 234], OperandSize::Qword)
}

#[test]
fn cmovb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 854489977, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 156, 137, 121, 123, 238, 50], OperandSize::Qword)
}

#[test]
fn cmovb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 237], OperandSize::Qword)
}

#[test]
fn cmovb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RDX, 738413113, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 154, 57, 74, 3, 44], OperandSize::Qword)
}

