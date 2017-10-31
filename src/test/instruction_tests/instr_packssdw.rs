use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 196], OperandSize::Dword)
}

#[test]
fn packssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 945082243, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 28, 141, 131, 207, 84, 56], OperandSize::Dword)
}

#[test]
fn packssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 201], OperandSize::Qword)
}

#[test]
fn packssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 42], OperandSize::Qword)
}

#[test]
fn packssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 239], OperandSize::Dword)
}

#[test]
fn packssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 38], OperandSize::Dword)
}

#[test]
fn packssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 192], OperandSize::Qword)
}

#[test]
fn packssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 36, 251], OperandSize::Qword)
}

