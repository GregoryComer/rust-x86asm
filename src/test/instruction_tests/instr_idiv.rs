use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn idiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 250], OperandSize::Word)
}

#[test]
fn idiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27689, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 185, 41, 108], OperandSize::Word)
}

#[test]
fn idiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Dword)
}

#[test]
fn idiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 56], OperandSize::Dword)
}

#[test]
fn idiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 249], OperandSize::Qword)
}

#[test]
fn idiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1636527371, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 60, 189, 11, 109, 139, 97], OperandSize::Qword)
}

#[test]
fn idiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 250], OperandSize::Qword)
}

#[test]
fn idiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(RDI, 206574692, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 191, 100, 20, 80, 12], OperandSize::Qword)
}

#[test]
fn idiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 249], OperandSize::Word)
}

#[test]
fn idiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(BP, 232, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 190, 232, 0], OperandSize::Word)
}

#[test]
fn idiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 253], OperandSize::Dword)
}

#[test]
fn idiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledDisplaced(EDI, Four, 2145665459, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 60, 189, 179, 65, 228, 127], OperandSize::Dword)
}

#[test]
fn idiv_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 251], OperandSize::Qword)
}

#[test]
fn idiv_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 60, 91], OperandSize::Qword)
}

#[test]
fn idiv_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 253], OperandSize::Word)
}

#[test]
fn idiv_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(BP, 47, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 126, 47], OperandSize::Word)
}

#[test]
fn idiv_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 252], OperandSize::Dword)
}

#[test]
fn idiv_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectDisplaced(EDI, 2006398159, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 191, 207, 52, 151, 119], OperandSize::Dword)
}

#[test]
fn idiv_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 250], OperandSize::Qword)
}

#[test]
fn idiv_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 96356344, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 188, 79, 248, 71, 190, 5], OperandSize::Qword)
}

#[test]
fn idiv_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 251], OperandSize::Qword)
}

#[test]
fn idiv_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IDIV, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 581738803, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 60, 253, 51, 161, 172, 34], OperandSize::Qword)
}

