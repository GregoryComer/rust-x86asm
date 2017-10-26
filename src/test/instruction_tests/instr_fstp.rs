use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 26], OperandSize::Word)
}

#[test]
fn fstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1474353684, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 28, 117, 20, 218, 224, 87], OperandSize::Dword)
}

#[test]
fn fstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1450320435, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 28, 205, 51, 34, 114, 86], OperandSize::Qword)
}

#[test]
fn fstp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 58], OperandSize::Word)
}

#[test]
fn fstp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledDisplaced(EDX, Two, 449534805, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 60, 85, 85, 91, 203, 26], OperandSize::Dword)
}

#[test]
fn fstp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 60, 159], OperandSize::Qword)
}

#[test]
fn fstp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 26], OperandSize::Word)
}

#[test]
fn fstp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(ESI, 624880481, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 158, 97, 235, 62, 37], OperandSize::Dword)
}

#[test]
fn fstp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 28, 155], OperandSize::Qword)
}

#[test]
fn fstp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 217], OperandSize::Word)
}

#[test]
fn fstp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 218], OperandSize::Dword)
}

#[test]
fn fstp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 222], OperandSize::Qword)
}

