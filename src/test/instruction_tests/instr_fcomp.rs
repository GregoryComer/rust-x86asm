use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectDisplaced(BX, 87, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 95, 87], OperandSize::Word)
}

#[test]
fn fcomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1375173372, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 28, 157, 252, 122, 247, 81], OperandSize::Dword)
}

#[test]
fn fcomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 28, 115], OperandSize::Qword)
}

#[test]
fn fcomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 218], OperandSize::Word)
}

#[test]
fn fcomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 223], OperandSize::Dword)
}

#[test]
fn fcomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 219], OperandSize::Qword)
}

#[test]
fn fcomp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 223, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 154, 223, 0], OperandSize::Word)
}

#[test]
fn fcomp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledDisplaced(EDX, Four, 879612948, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 28, 149, 20, 212, 109, 52], OperandSize::Dword)
}

#[test]
fn fcomp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 28, 138], OperandSize::Qword)
}

