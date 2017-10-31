use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 227], OperandSize::Dword)
}

#[test]
fn psignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 894110826, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 52, 253, 106, 12, 75, 53], OperandSize::Dword)
}

#[test]
fn psignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 245], OperandSize::Qword)
}

#[test]
fn psignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 23], OperandSize::Qword)
}

#[test]
fn psignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 192], OperandSize::Dword)
}

#[test]
fn psignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1106366726, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 156, 144, 6, 209, 241, 65], OperandSize::Dword)
}

#[test]
fn psignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 194], OperandSize::Qword)
}

#[test]
fn psignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 1782778352, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 169, 240, 9, 67, 106], OperandSize::Qword)
}

