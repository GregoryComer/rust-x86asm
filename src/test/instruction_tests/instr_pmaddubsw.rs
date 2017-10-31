use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 234], OperandSize::Dword)
}

#[test]
fn pmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1384195283, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 44, 253, 211, 36, 129, 82], OperandSize::Dword)
}

#[test]
fn pmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 241], OperandSize::Qword)
}

#[test]
fn pmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 848094510, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 60, 85, 46, 229, 140, 50], OperandSize::Qword)
}

#[test]
fn pmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 207], OperandSize::Dword)
}

#[test]
fn pmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 50], OperandSize::Dword)
}

#[test]
fn pmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 215], OperandSize::Qword)
}

#[test]
fn pmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 28, 242], OperandSize::Qword)
}

