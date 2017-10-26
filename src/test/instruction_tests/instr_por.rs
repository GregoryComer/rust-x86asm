use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn por_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 206], OperandSize::Dword)
}

#[test]
fn por_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 2092627957, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 20, 69, 245, 247, 186, 124], OperandSize::Dword)
}

#[test]
fn por_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 202], OperandSize::Qword)
}

#[test]
fn por_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 4, 242], OperandSize::Qword)
}

#[test]
fn por_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 254], OperandSize::Dword)
}

#[test]
fn por_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 58], OperandSize::Dword)
}

#[test]
fn por_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 253], OperandSize::Qword)
}

#[test]
fn por_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 1], OperandSize::Qword)
}

