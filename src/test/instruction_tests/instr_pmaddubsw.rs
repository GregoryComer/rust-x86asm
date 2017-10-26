use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 241], OperandSize::Dword)
}

#[test]
fn pmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 980053668, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 132, 128, 164, 110, 106, 58], OperandSize::Dword)
}

#[test]
fn pmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 233], OperandSize::Qword)
}

#[test]
fn pmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RSI, 2077239821, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 134, 13, 42, 208, 123], OperandSize::Qword)
}

#[test]
fn pmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 221], OperandSize::Dword)
}

#[test]
fn pmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 60, 128], OperandSize::Dword)
}

#[test]
fn pmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 233], OperandSize::Qword)
}

#[test]
fn pmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 962256515, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 60, 253, 131, 222, 90, 57], OperandSize::Qword)
}

