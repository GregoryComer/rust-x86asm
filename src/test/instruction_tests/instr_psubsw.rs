use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 254], OperandSize::Dword)
}

#[test]
fn psubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1616777842, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 12, 221, 114, 18, 94, 96], OperandSize::Dword)
}

#[test]
fn psubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 209], OperandSize::Qword)
}

#[test]
fn psubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1348702551, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 52, 133, 87, 145, 99, 80], OperandSize::Qword)
}

#[test]
fn psubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 211], OperandSize::Dword)
}

#[test]
fn psubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 20, 194], OperandSize::Dword)
}

#[test]
fn psubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 193], OperandSize::Qword)
}

#[test]
fn psubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 63], OperandSize::Qword)
}

