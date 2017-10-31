use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 211], OperandSize::Word)
}

#[test]
fn xadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 28555, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 144, 139, 111], OperandSize::Word)
}

#[test]
fn xadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 219], OperandSize::Dword)
}

#[test]
fn xadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 218784558, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 20, 77, 46, 99, 10, 13], OperandSize::Dword)
}

#[test]
fn xadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 219], OperandSize::Qword)
}

#[test]
fn xadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 2025914562, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 28, 213, 194, 0, 193, 120], OperandSize::Qword)
}

#[test]
fn xadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 211], OperandSize::Qword)
}

#[test]
fn xadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(RAX, 1770868975, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 152, 239, 80, 141, 105], OperandSize::Qword)
}

#[test]
fn xadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 242], OperandSize::Word)
}

#[test]
fn xadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 9053, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 184, 93, 35], OperandSize::Word)
}

#[test]
fn xadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 253], OperandSize::Dword)
}

#[test]
fn xadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 44, 250], OperandSize::Dword)
}

#[test]
fn xadd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 238], OperandSize::Qword)
}

#[test]
fn xadd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 28, 155], OperandSize::Qword)
}

#[test]
fn xadd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 230], OperandSize::Word)
}

#[test]
fn xadd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 40, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 80, 40], OperandSize::Word)
}

#[test]
fn xadd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 221], OperandSize::Dword)
}

#[test]
fn xadd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(ECX, 208656773, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 185, 133, 217, 111, 12], OperandSize::Dword)
}

#[test]
fn xadd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 252], OperandSize::Qword)
}

#[test]
fn xadd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 20, 192], OperandSize::Qword)
}

#[test]
fn xadd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 205], OperandSize::Qword)
}

#[test]
fn xadd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(RCX, 2057331587, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 169, 131, 99, 160, 122], OperandSize::Qword)
}

