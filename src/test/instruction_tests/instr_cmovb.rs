use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 220], OperandSize::Word)
}

#[test]
fn cmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(DI)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 63], OperandSize::Word)
}

#[test]
fn cmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 236], OperandSize::Dword)
}

#[test]
fn cmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 380283419, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 156, 150, 27, 170, 170, 22], OperandSize::Dword)
}

#[test]
fn cmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 239], OperandSize::Qword)
}

#[test]
fn cmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 36, 182], OperandSize::Qword)
}

#[test]
fn cmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 243], OperandSize::Word)
}

#[test]
fn cmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBP)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 44], OperandSize::Word)
}

#[test]
fn cmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 252], OperandSize::Dword)
}

#[test]
fn cmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1361456682, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 172, 251, 42, 46, 38, 81], OperandSize::Dword)
}

#[test]
fn cmovb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 236], OperandSize::Qword)
}

#[test]
fn cmovb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RBX, 899342716, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 163, 124, 225, 154, 53], OperandSize::Qword)
}

#[test]
fn cmovb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 214], OperandSize::Qword)
}

#[test]
fn cmovb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 24], OperandSize::Qword)
}

