use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 209], OperandSize::Dword)
}

#[test]
fn packssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EDI, 1387842577, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 151, 17, 204, 184, 82], OperandSize::Dword)
}

#[test]
fn packssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 226], OperandSize::Qword)
}

#[test]
fn packssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 14], OperandSize::Qword)
}

#[test]
fn packssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 236], OperandSize::Dword)
}

#[test]
fn packssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1806549473, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 20, 93, 225, 193, 173, 107], OperandSize::Dword)
}

#[test]
fn packssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 197], OperandSize::Qword)
}

#[test]
fn packssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 56], OperandSize::Qword)
}

