use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 11954, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 153, 178, 46], OperandSize::Word)
}

#[test]
fn fstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1032144169, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 156, 112, 41, 69, 133, 61], OperandSize::Dword)
}

#[test]
fn fstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(RDX, 2099862409, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 154, 137, 91, 41, 125], OperandSize::Qword)
}

#[test]
fn fstp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 58], OperandSize::Word)
}

#[test]
fn fstp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(EDI, 382136647, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 191, 71, 241, 198, 22], OperandSize::Dword)
}

#[test]
fn fstp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Indirect(RBX, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 59], OperandSize::Qword)
}

#[test]
fn fstp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 73, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 88, 73], OperandSize::Word)
}

#[test]
fn fstp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 28, 152], OperandSize::Dword)
}

#[test]
fn fstp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 25], OperandSize::Qword)
}

#[test]
fn fstp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 223], OperandSize::Word)
}

#[test]
fn fstp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 223], OperandSize::Dword)
}

#[test]
fn fstp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 218], OperandSize::Qword)
}

