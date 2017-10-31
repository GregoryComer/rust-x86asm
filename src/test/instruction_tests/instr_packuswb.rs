use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 216], OperandSize::Dword)
}

#[test]
fn packuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 726479860, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 172, 136, 244, 51, 77, 43], OperandSize::Dword)
}

#[test]
fn packuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 225], OperandSize::Qword)
}

#[test]
fn packuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1038330739, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 28, 181, 115, 171, 227, 61], OperandSize::Qword)
}

#[test]
fn packuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 208], OperandSize::Dword)
}

#[test]
fn packuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 40], OperandSize::Dword)
}

#[test]
fn packuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 196], OperandSize::Qword)
}

#[test]
fn packuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDI, 1044752665, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 135, 25, 169, 69, 62], OperandSize::Qword)
}

