use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 217], OperandSize::Word)
}

#[test]
fn cmovg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(BX, 85, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 111, 85], OperandSize::Word)
}

#[test]
fn cmovg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 238], OperandSize::Dword)
}

#[test]
fn cmovg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 16], OperandSize::Dword)
}

#[test]
fn cmovg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 221], OperandSize::Qword)
}

#[test]
fn cmovg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 380302144, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 172, 143, 64, 243, 170, 22], OperandSize::Qword)
}

#[test]
fn cmovg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 247], OperandSize::Word)
}

#[test]
fn cmovg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 16020, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 186, 148, 62], OperandSize::Word)
}

#[test]
fn cmovg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 209], OperandSize::Dword)
}

#[test]
fn cmovg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 938072814, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 28, 245, 238, 218, 233, 55], OperandSize::Dword)
}

#[test]
fn cmovg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 202], OperandSize::Qword)
}

#[test]
fn cmovg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 7363450, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 140, 138, 122, 91, 112, 0], OperandSize::Qword)
}

#[test]
fn cmovg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 247], OperandSize::Qword)
}

#[test]
fn cmovg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 60, 158], OperandSize::Qword)
}

