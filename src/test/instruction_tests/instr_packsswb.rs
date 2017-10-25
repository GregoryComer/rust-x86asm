use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packsswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 235], OperandSize::Dword)
}

#[test]
fn packsswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 11], OperandSize::Dword)
}

#[test]
fn packsswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 249], OperandSize::Qword)
}

#[test]
fn packsswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 198927319, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 52, 93, 215, 99, 219, 11], OperandSize::Qword)
}

#[test]
fn packsswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 240], OperandSize::Dword)
}

#[test]
fn packsswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 43], OperandSize::Dword)
}

#[test]
fn packsswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 212], OperandSize::Qword)
}

#[test]
fn packsswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 9], OperandSize::Qword)
}

