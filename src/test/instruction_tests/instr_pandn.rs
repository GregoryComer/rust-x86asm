use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 236], OperandSize::Dword)
}

#[test]
fn pandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1288913044, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 148, 112, 148, 64, 211, 76], OperandSize::Dword)
}

#[test]
fn pandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 215], OperandSize::Qword)
}

#[test]
fn pandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 20, 88], OperandSize::Qword)
}

#[test]
fn pandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 225], OperandSize::Dword)
}

#[test]
fn pandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 26], OperandSize::Dword)
}

#[test]
fn pandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 208], OperandSize::Qword)
}

#[test]
fn pandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 36, 78], OperandSize::Qword)
}

