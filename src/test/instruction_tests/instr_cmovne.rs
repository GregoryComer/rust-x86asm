use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 238], OperandSize::Word)
}

#[test]
fn cmovne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BX, 22130, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 191, 114, 86], OperandSize::Word)
}

#[test]
fn cmovne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 221], OperandSize::Dword)
}

#[test]
fn cmovne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(SI)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 51], OperandSize::Dword)
}

#[test]
fn cmovne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 237], OperandSize::Qword)
}

#[test]
fn cmovne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 60, 187], OperandSize::Qword)
}

#[test]
fn cmovne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 219], OperandSize::Word)
}

#[test]
fn cmovne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 157, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 139, 157, 0], OperandSize::Word)
}

#[test]
fn cmovne_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 220], OperandSize::Dword)
}

#[test]
fn cmovne_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 28, 250], OperandSize::Dword)
}

#[test]
fn cmovne_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 235], OperandSize::Qword)
}

#[test]
fn cmovne_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 30], OperandSize::Qword)
}

#[test]
fn cmovne_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 252], OperandSize::Qword)
}

#[test]
fn cmovne_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1279236521, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 44, 245, 169, 153, 63, 76], OperandSize::Qword)
}

