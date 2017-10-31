use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packsswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 236], OperandSize::Dword)
}

#[test]
fn packsswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 57], OperandSize::Dword)
}

#[test]
fn packsswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 227], OperandSize::Qword)
}

#[test]
fn packsswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 28, 115], OperandSize::Qword)
}

#[test]
fn packsswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 253], OperandSize::Dword)
}

#[test]
fn packsswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1947308979, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 180, 185, 179, 147, 17, 116], OperandSize::Dword)
}

#[test]
fn packsswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 197], OperandSize::Qword)
}

#[test]
fn packsswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 707803238, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 28, 253, 102, 56, 48, 42], OperandSize::Qword)
}

