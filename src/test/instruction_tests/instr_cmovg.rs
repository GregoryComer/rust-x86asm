use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 202], OperandSize::Word)
}

#[test]
fn cmovg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(BX, 231, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 159, 231, 0], OperandSize::Word)
}

#[test]
fn cmovg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 245], OperandSize::Dword)
}

#[test]
fn cmovg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EDX, 463223605, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 146, 53, 59, 156, 27], OperandSize::Dword)
}

#[test]
fn cmovg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 243], OperandSize::Qword)
}

#[test]
fn cmovg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 328530829, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 44, 85, 141, 251, 148, 19], OperandSize::Qword)
}

#[test]
fn cmovg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 204], OperandSize::Word)
}

#[test]
fn cmovg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 59], OperandSize::Word)
}

#[test]
fn cmovg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 238], OperandSize::Dword)
}

#[test]
fn cmovg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1423066840, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 12, 77, 216, 70, 210, 84], OperandSize::Dword)
}

#[test]
fn cmovg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 225], OperandSize::Qword)
}

#[test]
fn cmovg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 359693901, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 148, 191, 77, 126, 112, 21], OperandSize::Qword)
}

#[test]
fn cmovg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 244], OperandSize::Qword)
}

#[test]
fn cmovg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 54], OperandSize::Qword)
}

