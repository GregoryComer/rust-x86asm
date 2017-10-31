use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 11430, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 162, 166, 44], OperandSize::Word)
}

#[test]
fn fsub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(EBX, 1002405719, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 163, 87, 127, 191, 59], OperandSize::Dword)
}

#[test]
fn fsub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 392392335, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 164, 147, 143, 110, 99, 23], OperandSize::Qword)
}

#[test]
fn fsub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 230], OperandSize::Word)
}

#[test]
fn fsub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 230], OperandSize::Dword)
}

#[test]
fn fsub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 226], OperandSize::Qword)
}

#[test]
fn fsub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 34], OperandSize::Word)
}

#[test]
fn fsub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1196906181, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 36, 125, 197, 86, 87, 71], OperandSize::Dword)
}

#[test]
fn fsub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 36, 65], OperandSize::Qword)
}

#[test]
fn fsub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 233], OperandSize::Word)
}

#[test]
fn fsub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 235], OperandSize::Dword)
}

#[test]
fn fsub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 238], OperandSize::Qword)
}

