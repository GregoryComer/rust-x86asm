use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 213], OperandSize::Word)
}

#[test]
fn cmovnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 6654, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 153, 254, 25], OperandSize::Word)
}

#[test]
fn cmovnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 249], OperandSize::Dword)
}

#[test]
fn cmovnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(EDX, 768366234, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 162, 154, 86, 204, 45], OperandSize::Dword)
}

#[test]
fn cmovnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 253], OperandSize::Qword)
}

#[test]
fn cmovnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DI)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 57], OperandSize::Qword)
}

#[test]
fn cmovnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 207], OperandSize::Word)
}

#[test]
fn cmovnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 176, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 155, 176, 0], OperandSize::Word)
}

#[test]
fn cmovnge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 209], OperandSize::Dword)
}

#[test]
fn cmovnge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1651551324, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 60, 141, 92, 172, 112, 98], OperandSize::Dword)
}

#[test]
fn cmovnge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 239], OperandSize::Qword)
}

#[test]
fn cmovnge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1857408116, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 28, 133, 116, 204, 181, 110], OperandSize::Qword)
}

#[test]
fn cmovnge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 254], OperandSize::Qword)
}

#[test]
fn cmovnge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 62], OperandSize::Qword)
}

