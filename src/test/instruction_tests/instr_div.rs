use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn div_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 243], OperandSize::Word)
}

#[test]
fn div_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27360, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 177, 224, 106], OperandSize::Word)
}

#[test]
fn div_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Dword)
}

#[test]
fn div_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 640566338, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 180, 177, 66, 68, 46, 38], OperandSize::Dword)
}

#[test]
fn div_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 243], OperandSize::Qword)
}

#[test]
fn div_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RCX, 1280006676, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 177, 20, 90, 75, 76], OperandSize::Qword)
}

#[test]
fn div_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Qword)
}

#[test]
fn div_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 49], OperandSize::Qword)
}

#[test]
fn div_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 247], OperandSize::Word)
}

#[test]
fn div_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 52], OperandSize::Word)
}

#[test]
fn div_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 242], OperandSize::Dword)
}

#[test]
fn div_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 52, 158], OperandSize::Dword)
}

#[test]
fn div_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 245], OperandSize::Qword)
}

#[test]
fn div_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1717175643, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 52, 157, 91, 5, 90, 102], OperandSize::Qword)
}

#[test]
fn div_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 246], OperandSize::Word)
}

#[test]
fn div_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(BX, 14971, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 183, 123, 58], OperandSize::Word)
}

#[test]
fn div_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 246], OperandSize::Dword)
}

#[test]
fn div_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(EAX, 1416276344, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 176, 120, 169, 106, 84], OperandSize::Dword)
}

#[test]
fn div_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 243], OperandSize::Qword)
}

#[test]
fn div_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 54], OperandSize::Qword)
}

#[test]
fn div_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 244], OperandSize::Qword)
}

#[test]
fn div_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 55], OperandSize::Qword)
}

