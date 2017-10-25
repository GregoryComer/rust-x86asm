use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 245], OperandSize::Dword)
}

#[test]
fn psignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(EDI, 1315555497, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 135, 169, 200, 105, 78], OperandSize::Dword)
}

#[test]
fn psignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 220], OperandSize::Qword)
}

#[test]
fn psignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RAX, 557725977, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 128, 25, 57, 62, 33], OperandSize::Qword)
}

#[test]
fn psignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 208], OperandSize::Dword)
}

#[test]
fn psignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 22], OperandSize::Dword)
}

#[test]
fn psignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 238], OperandSize::Qword)
}

#[test]
fn psignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 709387009, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 176, 1, 99, 72, 42], OperandSize::Qword)
}

