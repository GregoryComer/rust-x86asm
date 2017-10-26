use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 242], OperandSize::Dword)
}

#[test]
fn pxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(ESI, 2029316330, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 190, 234, 232, 244, 120], OperandSize::Dword)
}

#[test]
fn pxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 236], OperandSize::Qword)
}

#[test]
fn pxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 48], OperandSize::Qword)
}

#[test]
fn pxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 245], OperandSize::Dword)
}

#[test]
fn pxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 44, 184], OperandSize::Dword)
}

#[test]
fn pxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 231], OperandSize::Qword)
}

#[test]
fn pxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 8], OperandSize::Qword)
}

