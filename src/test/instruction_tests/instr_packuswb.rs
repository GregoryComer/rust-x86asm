use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 200], OperandSize::Dword)
}

#[test]
fn packuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 9], OperandSize::Dword)
}

#[test]
fn packuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 223], OperandSize::Qword)
}

#[test]
fn packuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RCX, 2102658594, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 145, 34, 6, 84, 125], OperandSize::Qword)
}

#[test]
fn packuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 242], OperandSize::Dword)
}

#[test]
fn packuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1693242258, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 60, 133, 146, 211, 236, 100], OperandSize::Dword)
}

#[test]
fn packuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 237], OperandSize::Qword)
}

#[test]
fn packuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDX, 1445119114, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 186, 138, 196, 34, 86], OperandSize::Qword)
}

