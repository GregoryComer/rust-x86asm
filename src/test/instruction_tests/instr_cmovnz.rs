use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnz_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 245], OperandSize::Word)
}

#[test]
fn cmovnz_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(DI, 29848, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 157, 152, 116], OperandSize::Word)
}

#[test]
fn cmovnz_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 218], OperandSize::Dword)
}

#[test]
fn cmovnz_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 36, 152], OperandSize::Dword)
}

#[test]
fn cmovnz_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 231], OperandSize::Qword)
}

#[test]
fn cmovnz_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RCX, 1556537329, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 185, 241, 223, 198, 92], OperandSize::Qword)
}

#[test]
fn cmovnz_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 253], OperandSize::Word)
}

#[test]
fn cmovnz_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(DI, 3, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 109, 3], OperandSize::Word)
}

#[test]
fn cmovnz_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 223], OperandSize::Dword)
}

#[test]
fn cmovnz_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 542844813, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 164, 192, 141, 39, 91, 32], OperandSize::Dword)
}

#[test]
fn cmovnz_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 254], OperandSize::Qword)
}

#[test]
fn cmovnz_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RBX, 501398738, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 179, 210, 188, 226, 29], OperandSize::Qword)
}

#[test]
fn cmovnz_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 233], OperandSize::Qword)
}

#[test]
fn cmovnz_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RAX, 1072249388, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 136, 44, 58, 233, 63], OperandSize::Qword)
}

