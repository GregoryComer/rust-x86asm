use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 202], OperandSize::Word)
}

#[test]
fn xadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 11], OperandSize::Word)
}

#[test]
fn xadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 211], OperandSize::Dword)
}

#[test]
fn xadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 31], OperandSize::Dword)
}

#[test]
fn xadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 217], OperandSize::Qword)
}

#[test]
fn xadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 2037339933, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 148, 185, 29, 87, 111, 121], OperandSize::Qword)
}

#[test]
fn xadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 219], OperandSize::Qword)
}

#[test]
fn xadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 27], OperandSize::Qword)
}

#[test]
fn xadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 251], OperandSize::Word)
}

#[test]
fn xadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 105, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 123, 105], OperandSize::Word)
}

#[test]
fn xadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 231], OperandSize::Dword)
}

#[test]
fn xadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(ECX, 1378712254, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 137, 190, 122, 45, 82], OperandSize::Dword)
}

#[test]
fn xadd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 236], OperandSize::Qword)
}

#[test]
fn xadd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(RSI, Four, 656697727, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 60, 181, 127, 105, 36, 39], OperandSize::Qword)
}

#[test]
fn xadd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 253], OperandSize::Word)
}

#[test]
fn xadd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 45], OperandSize::Word)
}

#[test]
fn xadd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 255], OperandSize::Dword)
}

#[test]
fn xadd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(EDX, 198966247, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 154, 231, 251, 219, 11], OperandSize::Dword)
}

#[test]
fn xadd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 237], OperandSize::Qword)
}

#[test]
fn xadd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(RAX, Two, 47152786, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 12, 69, 146, 126, 207, 2], OperandSize::Qword)
}

#[test]
fn xadd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 214], OperandSize::Qword)
}

#[test]
fn xadd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1277492264, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 156, 202, 40, 252, 36, 76], OperandSize::Qword)
}

