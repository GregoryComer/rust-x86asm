use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 207], OperandSize::Dword)
}

#[test]
fn phminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EAX, 1930577973, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 160, 53, 72, 18, 115], OperandSize::Dword)
}

#[test]
fn phminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 224], OperandSize::Qword)
}

#[test]
fn phminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1757849307, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 188, 75, 219, 166, 198, 104], OperandSize::Qword)
}

