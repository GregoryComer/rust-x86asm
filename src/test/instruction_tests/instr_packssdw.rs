use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 205], OperandSize::Dword)
}

#[test]
fn packssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(ESI, 1201652466, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 134, 242, 194, 159, 71], OperandSize::Dword)
}

#[test]
fn packssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 220], OperandSize::Qword)
}

#[test]
fn packssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 49], OperandSize::Qword)
}

#[test]
fn packssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 247], OperandSize::Dword)
}

#[test]
fn packssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 62063711, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 28, 197, 95, 4, 179, 3], OperandSize::Dword)
}

#[test]
fn packssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 207], OperandSize::Qword)
}

#[test]
fn packssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 52, 176], OperandSize::Qword)
}

