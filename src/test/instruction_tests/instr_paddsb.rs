use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 243], OperandSize::Dword)
}

#[test]
fn paddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 59], OperandSize::Dword)
}

#[test]
fn paddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 235], OperandSize::Qword)
}

#[test]
fn paddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 22], OperandSize::Qword)
}

#[test]
fn paddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 199], OperandSize::Dword)
}

#[test]
fn paddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 553099140, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 176, 132, 159, 247, 32], OperandSize::Dword)
}

#[test]
fn paddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 194], OperandSize::Qword)
}

#[test]
fn paddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 60, 130], OperandSize::Qword)
}

