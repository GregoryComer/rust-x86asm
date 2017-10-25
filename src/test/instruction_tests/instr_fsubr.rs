use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(BP, 41, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 110, 41], OperandSize::Word)
}

#[test]
fn fsubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1725762296, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 44, 197, 248, 10, 221, 102], OperandSize::Dword)
}

#[test]
fn fsubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1514687995, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 172, 247, 251, 77, 72, 90], OperandSize::Qword)
}

#[test]
fn fsubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 238], OperandSize::Word)
}

#[test]
fn fsubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 233], OperandSize::Dword)
}

#[test]
fn fsubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 235], OperandSize::Qword)
}

#[test]
fn fsubr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Indirect(SI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 44], OperandSize::Word)
}

#[test]
fn fsubr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 802031910, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 172, 142, 38, 9, 206, 47], OperandSize::Dword)
}

#[test]
fn fsubr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 996660622, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 172, 86, 142, 213, 103, 59], OperandSize::Qword)
}

#[test]
fn fsubr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 230], OperandSize::Word)
}

#[test]
fn fsubr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 225], OperandSize::Dword)
}

#[test]
fn fsubr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 231], OperandSize::Qword)
}

