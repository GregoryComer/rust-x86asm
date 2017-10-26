use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 236], OperandSize::Dword)
}

#[test]
fn packssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 63], OperandSize::Dword)
}

#[test]
fn packssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 234], OperandSize::Qword)
}

#[test]
fn packssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 12, 186], OperandSize::Qword)
}

#[test]
fn packssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 228], OperandSize::Dword)
}

#[test]
fn packssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 0], OperandSize::Dword)
}

#[test]
fn packssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 252], OperandSize::Qword)
}

#[test]
fn packssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 46], OperandSize::Qword)
}

